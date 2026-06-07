<script lang="ts">
	let {
		value = $bindable(''),
		readonly = false,
		errorLine = null
	}: {
		value: string;
		readonly?: boolean;
		errorLine?: number | null;
	} = $props();

	let textarea: HTMLTextAreaElement;
	let pre: HTMLPreElement;
	let lineNumbersContainer: HTMLDivElement;

	// Line numbers calculation
	let lines = $derived(value.split('\n'));

	// Syntax highlighted content
	let highlightedContent = $derived(highlight(value) + (value.endsWith('\n') ? '\n' : ''));

	// Svelte action to safely render HTML without {@html} lint issues
	function renderHighlight(node: HTMLElement, content: string) {
		const update = (c: string) => {
			node.innerHTML = c;
		};
		update(content);
		return { update };
	}

	// Simple syntax highlighter for JSON-like text
	function highlight(text: string) {
		if (!text) return '';

		// Escape HTML entities to prevent XSS and incorrect rendering
		let escaped = text
			.replace(/&/g, '&amp;')
			.replace(/</g, '&lt;')
			.replace(/>/g, '&gt;')
			.replace(/"/g, '&quot;')
			.replace(/'/g, '&#039;');

		// Regex-based JSON highlighting
		return escaped.replace(
			/(&quot;(?:\\[\s\S]|(?!&quot;)[\s\S])*&quot;(\s*:)?)|(\b(true|false|null)\b)|(\b-?\d+(?:\.\d*)?(?:[eE][+-]?\d+)?\b)/g,
			function (match) {
				let cls = 'text-blue-600'; // Default: numbers
				if (/^&quot;/.test(match)) {
					if (/:$/.test(match)) {
						cls = 'text-indigo-800 font-bold'; // Keys
					} else {
						cls = 'text-emerald-600'; // Strings
					}
				} else if (/true|false/.test(match)) {
					cls = 'text-amber-600 font-semibold'; // Booleans
				} else if (/null/.test(match)) {
					cls = 'text-slate-400 italic'; // Null
				}
				return '<span class="' + cls + '">' + match + '</span>';
			}
		);
	}

	function handleScroll() {
		if (textarea && pre && lineNumbersContainer) {
			pre.scrollTop = textarea.scrollTop;
			pre.scrollLeft = textarea.scrollLeft;
			lineNumbersContainer.scrollTop = textarea.scrollTop;
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Tab') {
			e.preventDefault();
			const start = textarea.selectionStart;
			const end = textarea.selectionEnd;

			value = value.substring(0, start) + '  ' + value.substring(end);

			// Reset cursor position after state update
			setTimeout(() => {
				textarea.selectionStart = textarea.selectionEnd = start + 2;
			}, 0);
		}
	}
</script>

<div
	class="flex h-full w-full overflow-hidden rounded-3xl border border-slate-200 bg-white shadow-sm transition-all focus-within:ring-2 focus-within:ring-blue-500"
>
	<!-- Line Numbers -->
	<div
		bind:this={lineNumbersContainer}
		class="w-12 overflow-hidden bg-slate-50 text-right font-mono text-xs text-slate-300 select-none"
		style="padding-right: 0.75rem;"
	>
		<div>
			{#each lines, line_number (line_number)}
				<div
					class="h-5 leading-5 {errorLine === line_number + 1
						? 'bg-red-100 font-bold text-red-400'
						: ''}"
				>
					{line_number + 1}
				</div>
			{/each}
		</div>
	</div>

	<!-- Editor Area -->
	<div class="relative flex-1 overflow-hidden bg-white">
		<pre
			bind:this={pre}
			use:renderHighlight={highlightedContent}
			aria-hidden="true"
			class="pointer-events-none absolute inset-0 z-0 m-0 overflow-hidden font-mono text-sm break-normal whitespace-pre"></pre>

		<textarea
			bind:this={textarea}
			bind:value
			{readonly}
			onscroll={handleScroll}
			onkeydown={handleKeydown}
			spellcheck="false"
			placeholder={readonly ? '' : '여기에 JSON을 입력하세요...'}
			class="relative z-10 m-0 block h-full w-full resize-none overflow-auto border-none bg-transparent p-0 font-mono text-sm leading-5 break-normal whitespace-pre text-transparent caret-slate-900 focus:outline-none"
		></textarea>
	</div>
</div>

<style>
	/* Ensure typography is identical across layers */
	pre,
	textarea {
		font-family:
			'JetBrains Mono', 'Fira Code', 'Menlo', 'Monaco', 'Consolas', 'Liberation Mono',
			'Courier New', monospace;
		line-height: 1.25rem; /* Exactly 20px to match line numbers */
	}

	textarea::placeholder {
		color: #94a3b8; /* slate-400 */
		-webkit-text-fill-color: #94a3b8;
	}

	/* Custom scrollbar for better look */
	textarea::-webkit-scrollbar {
		width: 10px;
		height: 10px;
	}
	textarea::-webkit-scrollbar-track {
		background: transparent;
	}
	textarea::-webkit-scrollbar-thumb {
		background: #e2e8f0;
		border-radius: 10px;
		border: 2px solid white;
	}
	textarea::-webkit-scrollbar-thumb:hover {
		background: #cbd5e1;
	}
</style>
