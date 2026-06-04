<script lang="ts">
	import { resolve } from '$app/paths';

	// Unicode mapping for script (cursive) characters
	// Math Alphanumeric Symbols - Script (Normal)
	const cursiveMap: Record<string, string> = {
		A: '𝒜',
		B: 'ℬ',
		C: '𝒞',
		D: '𝒟',
		E: 'ℰ',
		F: 'ℱ',
		G: '𝒢',
		H: 'ℋ',
		I: 'ℐ',
		J: '𝒥',
		K: '𝒦',
		L: 'ℒ',
		M: 'ℳ',
		N: '𝒩',
		O: '𝒪',
		P: '𝒫',
		Q: '𝒬',
		R: 'ℛ',
		S: '𝒮',
		T: '𝒯',
		U: '𝒰',
		V: '𝒱',
		W: '𝒲',
		X: '𝒳',
		Y: '𝒴',
		Z: '𝒵',
		a: '𝒶',
		b: '𝒷',
		c: '𝒸',
		d: '𝒹',
		e: 'ℯ',
		f: '𝒻',
		g: 'ℊ',
		h: '𝒽',
		i: '𝒾',
		j: '𝒿',
		k: '𝓀',
		l: '𝓁',
		m: '𝓂',
		n: '𝓃',
		o: 'ℴ',
		p: '𝓅',
		q: '𝓆',
		r: '𝓇',
		s: '𝓈',
		t: '𝓉',
		u: '𝓊',
		v: '𝓋',
		w: '𝓌',
		x: '𝓍',
		y: '𝓎',
		z: '𝓏'
	};

	let inputText = $state('');
	let copied = $state(false);

	let outputText = $derived(
		inputText
			.split('')
			.map((char) => cursiveMap[char] || char)
			.join('')
	);

	async function copyToClipboard() {
		if (!outputText) return;
		try {
			await navigator.clipboard.writeText(outputText);
			copied = true;
			setTimeout(() => {
				copied = false;
			}, 2000);
		} catch (err) {
			console.error('Failed to copy: ', err);
		}
	}
</script>

<div class="min-h-screen bg-[#f8fafc] font-sans text-slate-900">
	<header class="sticky top-0 z-20 border-b border-slate-200 bg-white">
		<div class="mx-auto flex max-w-4xl items-center justify-between px-6 py-4">
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
					<div class="text-xl">✍️</div>
					<h1 class="text-lg font-bold tracking-tight text-slate-800">ℒ𝓊𝓍𝓊𝓇𝓎</h1>
				</div>
			</div>
		</div>
	</header>

	<main class="mx-auto max-w-4xl px-6 py-12">
		<div class="mb-10 text-center">
			<h2 class="mb-3 text-3xl font-black text-slate-900">ℒ𝓊𝓍𝓊𝓇𝓎</h2>
			<p class="text-slate-500">영문 텍스트를 멋진 필기체 유니코드로 변환해 보세요.</p>
		</div>

		<div class="grid gap-8">
			<!-- Input Section -->
			<div class="space-y-3">
				<label for="input" class="ml-1 block text-sm font-bold text-slate-700"
					>입력 (English Only)</label
				>
				<textarea
					id="input"
					bind:value={inputText}
					placeholder="여기에 영문 텍스트를 입력하세요..."
					class="h-40 w-full resize-none rounded-3xl border border-slate-200 bg-white p-6 text-lg shadow-sm transition-all focus:border-transparent focus:ring-2 focus:ring-indigo-500 focus:outline-none"
				></textarea>
			</div>

			<!-- Output Section -->
			<div class="space-y-3">
				<div class="ml-1 flex items-center justify-between">
					<label for="output" class="block text-sm font-bold text-slate-700"
						>변환 결과 (Cursive text)</label
					>
					{#if inputText}
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
								복사 완료!
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
				<div
					id="output"
					class="min-h-40 w-full rounded-3xl border border-indigo-100 bg-indigo-50/30 p-6 font-serif text-2xl break-all text-indigo-900"
				>
					{outputText || '변환된 결과가 여기에 표시됩니다.'}
				</div>
			</div>

			<!-- Guide / Tips -->
			<div class="mt-8 rounded-3xl border border-slate-100 bg-white p-6 shadow-sm">
				<h3 class="mb-3 flex items-center gap-2 font-bold text-slate-800">
					<span class="text-indigo-500">💡</span> 사용 팁
				</h3>
				<ul class="space-y-2 text-sm leading-relaxed text-slate-500">
					<li>• 변환된 글자는 폰트가 아닌 <strong>특수 유니코드 문자</strong>입니다.</li>
					<li>• 인스타그램 프로필, 카카오톡 메시지, 트위터(X) 등 어디서든 사용할 수 있습니다.</li>
					<li>• 영문 대문자와 소문자만 변환되며, 숫자와 기호는 유지됩니다.</li>
				</ul>
			</div>
		</div>
	</main>

	<footer class="mt-auto py-10 text-center text-xs text-slate-400">
		© 2026 Klutter Luxury Text Converter.
	</footer>
</div>
