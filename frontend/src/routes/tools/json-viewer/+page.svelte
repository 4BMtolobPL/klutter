<script lang="ts">
	import { resolve } from '$app/paths';

	let inputText = $state('');
	let copied = $state(false);

	let result = $derived.by(() => {
		if (!inputText.trim()) {
			return { outputText: '', errorMsg: '' };
		}
		try {
			const parsed = JSON.parse(inputText);
			return { outputText: JSON.stringify(parsed, null, 2), errorMsg: '' };
		} catch (e) {
			return { outputText: '', errorMsg: (e as Error).message };
		}
	});

	async function copyToClipboard() {
		if (!result.outputText) return;
		try {
			await navigator.clipboard.writeText(result.outputText);
			copied = true;
			setTimeout(() => {
				copied = false;
			}, 2000);
		} catch (err) {
			console.error('Failed to copy: ', err);
		}
	}

	function clearAll() {
		inputText = '';
	}

	function beautify() {
		if (result.outputText) {
			inputText = result.outputText;
		}
	}
</script>

<div class="min-h-screen bg-[#f8fafc] font-sans text-slate-900">
	<header class="sticky top-0 z-20 border-b border-slate-200 bg-white">
		<div class="mx-auto flex max-w-6xl items-center justify-between px-6 py-4">
			<div class="flex items-center gap-4">
				<a
					href={resolve('/')}
					class="rounded-full p-2 text-slate-500 transition-colors hover:bg-slate-100"
					aria-label="Back to home"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="24"
						height="24"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"><path d="m15 18-6-6 6-6" /></svg
					>
				</a>
				<div class="flex items-center gap-2">
					<div class="text-xl">{'{}'}</div>
					<h1 class="text-lg font-bold tracking-tight text-slate-800">JSON 포매터</h1>
				</div>
			</div>
		</div>
	</header>

	<main class="mx-auto max-w-6xl px-6 py-12">
		<div class="mb-10 text-center">
			<h2 class="mb-3 text-3xl font-black text-slate-900">JSON 포매터</h2>
			<p class="text-slate-500">복잡한 JSON 데이터를 읽기 좋게 정리하고 검증하세요.</p>
		</div>

		<div class="grid grid-cols-1 gap-8 lg:grid-cols-2">
			<!-- Input Section -->
			<div class="flex flex-col space-y-3">
				<div class="flex items-center justify-between px-1">
					<label for="input" class="text-sm font-bold text-slate-700">입력 (Raw JSON)</label>
					<button
						onclick={clearAll}
						class="text-xs font-semibold text-slate-400 transition-colors hover:text-slate-600"
					>
						지우기
					</button>
				</div>
				<textarea
					id="input"
					bind:value={inputText}
					placeholder="여기에 JSON 데이터를 붙여넣으세요..."
					class="h-125 w-full resize-none rounded-3xl border border-slate-200 bg-white p-6 font-mono text-sm shadow-sm transition-all focus:border-transparent focus:ring-2 focus:ring-blue-500 focus:outline-none"
				></textarea>
			</div>

			<!-- Output Section -->
			<div class="flex flex-col space-y-3">
				<div class="flex items-center justify-between px-1">
					<label for="output" class="text-sm font-bold text-slate-700">결과 (Formatted JSON)</label>
					<div class="flex gap-2">
						{#if result.outputText}
							<button
								onclick={beautify}
								class="rounded-full bg-slate-100 px-3 py-1.5 text-xs font-bold text-slate-600 transition-all hover:bg-slate-200"
							>
								입력창에 반영
							</button>
							<button
								onclick={copyToClipboard}
								class="flex items-center gap-1.5 rounded-full px-3 py-1.5 text-xs font-bold transition-all {copied
									? 'bg-emerald-500 text-white shadow-lg shadow-emerald-200'
									: 'bg-slate-900 text-white hover:bg-slate-800'}"
							>
								{#if copied}
									<svg
										xmlns="http://www.w3.org/2000/svg"
										class="h-3 w-3"
										fill="none"
										viewBox="0 0 24 24"
										stroke="currentColor"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="3"
											d="M5 13l4 4L19 7"
										/>
									</svg>
									복사됨
								{:else}
									<svg
										xmlns="http://www.w3.org/2000/svg"
										class="h-3 w-3"
										fill="none"
										viewBox="0 0 24 24"
										stroke="currentColor"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3"
										/>
									</svg>
									복사하기
								{/if}
							</button>
						{/if}
					</div>
				</div>
				<div
					id="output"
					class="relative h-125 w-full overflow-hidden rounded-3xl border {result.errorMsg
						? 'border-red-200 bg-red-50/30'
						: 'border-blue-100 bg-blue-50/30'} shadow-inner"
				>
					{#if result.errorMsg}
						<div class="p-6 text-sm font-medium text-red-500">
							<div class="mb-2 flex items-center gap-2 font-bold uppercase">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="16"
									height="16"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
									><circle cx="12" cy="12" r="10" /><line x1="12" x2="12" y1="8" y2="12" /><line
										x1="12"
										x2="12.01"
										y1="16"
										y2="16"
									/></svg
								>
								Invalid JSON
							</div>
							{result.errorMsg}
						</div>
					{:else if result.outputText}
						<pre
							class="h-full overflow-auto p-6 font-mono text-sm whitespace-pre text-blue-900">{result.outputText}</pre>
					{:else}
						<div class="flex h-full items-center justify-center text-sm text-slate-400">
							결과가 여기에 표시됩니다.
						</div>
					{/if}
				</div>
			</div>
		</div>

		<!-- Guide -->
		<div class="mt-12 rounded-3xl border border-slate-100 bg-white p-8 shadow-sm">
			<h3 class="mb-4 flex items-center gap-2 font-bold text-slate-800">
				<span class="text-blue-500">💡</span> 기능 안내
			</h3>
			<div class="grid gap-6 md:grid-cols-2">
				<ul class="space-y-2 text-sm leading-relaxed text-slate-500">
					<li>• <strong>자동 포매팅:</strong> JSON을 입력하는 즉시 2칸 들여쓰기로 정리합니다.</li>
					<li>
						• <strong>유효성 검사:</strong> 잘못된 JSON 형식인 경우 즉시 에러 위치를 알려줍니다.
					</li>
				</ul>
				<ul class="space-y-2 text-sm leading-relaxed text-slate-500">
					<li>
						• <strong>클립보드 복사:</strong> 버튼 클릭 한 번으로 결과물을 간편하게 복사하세요.
					</li>
					<li>
						• <strong>안전한 처리:</strong> 모든 데이터는 브라우저 내에서만 처리되며 서버로 전송되지 않습니다.
					</li>
				</ul>
			</div>
		</div>
	</main>

	<footer class="mt-auto py-10 text-center text-xs text-slate-400">
		© 2026 Klutter JSON Formatter Tool.
	</footer>
</div>

<style>
	/* Custom scrollbar for output area */
	pre::-webkit-scrollbar {
		width: 8px;
		height: 8px;
	}
	pre::-webkit-scrollbar-track {
		background: transparent;
	}
	pre::-webkit-scrollbar-thumb {
		background: #dbeafe;
		border-radius: 10px;
	}
	pre::-webkit-scrollbar-thumb:hover {
		background: #bfdbfe;
	}
</style>
