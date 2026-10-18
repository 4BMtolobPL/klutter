<script lang="ts">
	import { resolve } from '$app/paths';

	type Base = 'HEX' | 'DEC' | 'BIN' | 'OCT';

	const bases: Base[] = ['HEX', 'DEC', 'BIN', 'OCT'];

	const baseInfo: Record<Base, { label: string; placeholder: string; digits: RegExp }> = {
		HEX: { label: '16진수 (HEX)', placeholder: '예: 1A2F', digits: /^[0-9a-fA-F]+$/ },
		DEC: { label: '10진수 (DEC)', placeholder: '예: 6703', digits: /^[0-9]+$/ },
		BIN: { label: '2진수 (BIN)', placeholder: '예: 1101000101111', digits: /^[01]+$/ },
		OCT: { label: '8진수 (OCT)', placeholder: '예: 15057', digits: /^[0-7]+$/ }
	};

	// --- Base conversion ---
	let convertInput = $state('');
	let convertBase = $state<Base>('HEX');
	let copiedKey = $state('');

	let convertResult = $derived.by((): { value: bigint | null; error: string } => {
		const raw = convertInput
			.trim()
			.replace(/^0[xX]/, '')
			.replace(/^0[bBoO]/, '');
		if (!raw) {
			return { value: null, error: '' };
		}
		if (!baseInfo[convertBase].digits.test(raw)) {
			return { value: null, error: `${convertBase} 형식에 맞지 않는 문자가 포함되어 있습니다.` };
		}
		try {
			return { value: BigInt(parseIntOrRaw(raw, convertBase)), error: '' };
		} catch {
			return { value: null, error: '변환할 수 없는 값입니다.' };
		}
	});

	let convertValue = $derived(convertResult.value);
	let convertError = $derived(convertResult.error);

	let convertResults = $derived.by((): Record<Base, string> => {
		const empty = { HEX: '', DEC: '', BIN: '', OCT: '' } as Record<Base, string>;
		if (convertValue === null) return empty;
		const v = convertValue;
		return {
			HEX: v.toString(16).toUpperCase(),
			DEC: v.toString(10),
			BIN: v.toString(2),
			OCT: v.toString(8)
		};
	});

	function parseIntOrRaw(raw: string, base: Base): string {
		// BigInt can parse 0-prefixed strings for 2/8/10/16 directly
		if (base === 'HEX') return '0x' + raw;
		if (base === 'BIN') return '0b' + raw;
		if (base === 'OCT') return '0o' + raw;
		return raw;
	}

	function setConvertBase(base: Base) {
		convertBase = base;
	}

	// --- Arithmetic ---
	let exprInput = $state('');

	let calcResult = $derived.by((): { value: bigint | null; error: string } => {
		const raw = exprInput.trim();
		if (!raw) {
			return { value: null, error: '' };
		}
		try {
			return { value: evaluateExpression(raw), error: '' };
		} catch (e) {
			return { value: null, error: (e as Error).message };
		}
	});

	let calcValue = $derived(calcResult.value);
	let calcError = $derived(calcResult.error);

	let calcResults = $derived.by((): Record<Base, string> => {
		const empty = { HEX: '', DEC: '', BIN: '', OCT: '' } as Record<Base, string>;
		if (calcValue === null) return empty;
		const v = calcValue;
		return {
			HEX: v.toString(16).toUpperCase(),
			DEC: v.toString(10),
			BIN: v.toString(2),
			OCT: v.toString(8)
		};
	});

	type Token = { type: 'num'; value: bigint } | { type: 'op'; value: string };

	function tokenize(input: string): Token[] {
		const tokens: Token[] = [];
		let i = 0;
		const s = input.replace(/\s+/g, '');
		while (i < s.length) {
			const c = s[i];
			if (c === '+' || c === '-' || c === '*' || c === '/' || c === '(' || c === ')') {
				tokens.push({ type: 'op', value: c });
				i++;
				continue;
			}
			// number: 0x… | 0b… | 0o… | decimal | bare hex (letters a-f allowed)
			let num = '';
			if (c === '0' && (s[i + 1] === 'x' || s[i + 1] === 'X')) {
				num = s.slice(i, i + 2);
				i += 2;
				while (i < s.length && /[0-9a-fA-F]/.test(s[i])) {
					num += s[i];
					i++;
				}
			} else if (c === '0' && (s[i + 1] === 'b' || s[i + 1] === 'B')) {
				num = s.slice(i, i + 2);
				i += 2;
				while (i < s.length && /[01]/.test(s[i])) {
					num += s[i];
					i++;
				}
			} else if (c === '0' && (s[i + 1] === 'o' || s[i + 1] === 'O')) {
				num = s.slice(i, i + 2);
				i += 2;
				while (i < s.length && /[0-7]/.test(s[i])) {
					num += s[i];
					i++;
				}
			} else {
				while (i < s.length && /[0-9a-fA-F]/.test(s[i])) {
					num += s[i];
					i++;
				}
			}
			if (!num) {
				throw new Error(`알 수 없는 문자: "${c}"`);
			}
			// bare (no prefix) numbers with a-f letters are treated as hex; pure digits as decimal
			const hasAlpha = /[a-fA-F]/.test(num);
			const toParse =
				num.startsWith('0x') || num.startsWith('0X')
					? num
					: num.startsWith('0b') || num.startsWith('0B')
						? num
						: num.startsWith('0o') || num.startsWith('0O')
							? num
							: hasAlpha
								? '0x' + num
								: num;
			tokens.push({ type: 'num', value: BigInt(toParse) });
		}
		return tokens;
	}

	function evaluateExpression(input: string): bigint {
		const tokens = tokenize(input);
		let pos = 0;

		function peek(): Token | undefined {
			return tokens[pos];
		}
		function next(): Token {
			const t = tokens[pos];
			if (!t) throw new Error('수식이 끝나지 않았습니다.');
			pos++;
			return t;
		}
		function expectOp(value: string) {
			const t = next();
			if (t.type !== 'op' || t.value !== value) {
				throw new Error(`잘못된 수식입니다. "${value}"이(가) 필요합니다.`);
			}
		}

		function parseExpr(): bigint {
			let left = parseTerm();
			while (peek()?.type === 'op' && (peek()!.value === '+' || peek()!.value === '-')) {
				const op = (next() as { value: string }).value;
				const right = parseTerm();
				left = op === '+' ? left + right : left - right;
			}
			return left;
		}

		function parseTerm(): bigint {
			let left = parseFactor();
			while (peek()?.type === 'op' && (peek()!.value === '*' || peek()!.value === '/')) {
				const op = (next() as { value: string }).value;
				const right = parseFactor();
				if (op === '/' && right === 0n) {
					throw new Error('0으로 나눌 수 없습니다.');
				}
				left = op === '*' ? left * right : left / right;
			}
			return left;
		}

		function parseFactor(): bigint {
			const t = peek();
			if (!t) throw new Error('수식이 올바르지 않습니다.');
			if (t.type === 'op' && t.value === '-') {
				next();
				return -parseFactor();
			}
			if (t.type === 'op' && t.value === '+') {
				next();
				return parseFactor();
			}
			if (t.type === 'op' && t.value === '(') {
				next();
				const v = parseExpr();
				expectOp(')');
				return v;
			}
			if (t.type === 'num') {
				next();
				return t.value;
			}
			throw new Error('숫자가 필요한 위치에 잘못된 토큰이 있습니다.');
		}

		const result = parseExpr();
		if (pos !== tokens.length) {
			throw new Error('수식 끝에 알 수 없는 토큰이 있습니다.');
		}
		return result;
	}

	// --- Paste ---
	function handlePaste(e: ClipboardEvent & { currentTarget: EventTarget & HTMLInputElement }) {
		const pasted = e.clipboardData?.getData('text') ?? '';
		if (!pasted.includes(',')) return;
		e.preventDefault();
		const input = e.currentTarget;
		const cleaned = pasted.replace(/,/g, '');
		const start = input.selectionStart ?? input.value.length;
		const end = input.selectionEnd ?? input.value.length;
		input.value = input.value.slice(0, start) + cleaned + input.value.slice(end);
		const caret = start + cleaned.length;
		input.setSelectionRange(caret, caret);
		input.dispatchEvent(new Event('input', { bubbles: true }));
	}

	// --- Clipboard ---
	async function copyToClipboard(text: string, key: string) {
		if (!text) return;
		try {
			await navigator.clipboard.writeText(text);
			copiedKey = key;
			setTimeout(() => {
				if (copiedKey === key) copiedKey = '';
			}, 2000);
		} catch (err) {
			console.error('Failed to copy: ', err);
		}
	}

	function clearAll() {
		convertInput = '';
		exprInput = '';
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
					<div class="text-xl">0x</div>
					<h1 class="text-lg font-bold tracking-tight text-slate-800">HEX 계산기</h1>
				</div>
			</div>
			<button
				onclick={clearAll}
				class="text-xs font-semibold text-slate-400 transition-colors hover:text-slate-600"
			>
				전체 지우기
			</button>
		</div>
	</header>

	<main class="mx-auto max-w-4xl px-6 py-12">
		<div class="mb-10 text-center">
			<h2 class="mb-3 text-3xl font-black text-slate-900">HEX 계산기</h2>
			<p class="text-slate-500">16진수 변환과 사칙연산을 한 번에 처리하세요.</p>
		</div>

		<div class="grid gap-10">
			<!-- Base Conversion Section -->
			<section class="space-y-4">
				<h3 class="ml-1 text-lg font-bold text-slate-800">베이스 변환</h3>

				<div class="space-y-3">
					<label for="convert-input" class="ml-1 block text-sm font-bold text-slate-700">
						{baseInfo[convertBase].label} 입력
					</label>
					<div class="flex gap-2">
						<input
							id="convert-input"
							type="text"
							bind:value={convertInput}
							onpaste={handlePaste}
							placeholder={baseInfo[convertBase].placeholder}
							class="w-full rounded-3xl border border-slate-200 bg-white px-6 py-4 font-mono text-lg shadow-sm transition-all focus:border-transparent focus:ring-2 focus:ring-indigo-500 focus:outline-none"
						/>
					</div>
					<div class="flex gap-2">
						{#each bases as base (base)}
							<button
								onclick={() => setConvertBase(base)}
								class="rounded-full px-4 py-2 text-xs font-bold transition-all {convertBase === base
									? 'bg-slate-900 text-white shadow-lg shadow-slate-200'
									: 'border border-slate-200 bg-white text-slate-500 hover:bg-slate-100'}"
							>
								{base}
							</button>
						{/each}
					</div>
					{#if convertError}
						<div
							class="rounded-2xl border border-red-200 bg-red-50/30 px-5 py-3 text-sm font-medium text-red-500"
						>
							{convertError}
						</div>
					{/if}
				</div>

				<div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
					{#each bases.filter((b) => b !== convertBase) as base (base)}
						<div class="rounded-3xl border border-slate-200 bg-white p-5 shadow-sm">
							<div class="mb-2 flex items-center justify-between">
								<span class="text-xs font-bold tracking-widest text-slate-400 uppercase">
									{baseInfo[base].label}
								</span>
								<button
									onclick={() => copyToClipboard(convertResults[base], 'convert-' + base)}
									class="flex items-center gap-1.5 rounded-full px-3 py-1.5 text-xs font-bold transition-all {copiedKey ===
									'convert-' + base
										? 'bg-emerald-500 text-white shadow-lg shadow-emerald-200'
										: 'bg-slate-100 text-slate-600 hover:bg-slate-200'}"
								>
									{#if copiedKey === 'convert-' + base}
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
										복사하기
									{/if}
								</button>
							</div>
							<div class="font-mono text-lg break-all text-slate-800">
								{convertResults[base] || '—'}
							</div>
						</div>
					{/each}
				</div>
			</section>

			<!-- Arithmetic Section -->
			<section class="space-y-4">
				<h3 class="ml-1 text-lg font-bold text-slate-800">사칙연산</h3>

				<div class="space-y-3">
					<label for="expr-input" class="ml-1 block text-sm font-bold text-slate-700"
						>수식 입력</label
					>
					<input
						id="expr-input"
						type="text"
						bind:value={exprInput}
						onpaste={handlePaste}
						placeholder="예: FF + 0b1010 * (12 - 3)"
						class="w-full rounded-3xl border border-slate-200 bg-white px-6 py-4 font-mono text-lg shadow-sm transition-all focus:border-transparent focus:ring-2 focus:ring-indigo-500 focus:outline-none"
					/>
					<p class="ml-1 text-xs text-slate-400">
						HEX는 기본값입니다. 0b 접두어=2진수, 0o 접두어=8진수, 숫자만=10진수, 소문자 a-f 포함 시
						HEX로 인식합니다. 괄호( ) 사용 가능.
					</p>
					{#if calcError}
						<div
							class="rounded-2xl border border-red-200 bg-red-50/30 px-5 py-3 text-sm font-medium text-red-500"
						>
							{calcError}
						</div>
					{/if}
				</div>

				<div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
					{#each bases as base (base)}
						<div class="rounded-3xl border border-slate-200 bg-white p-5 shadow-sm">
							<div class="mb-2 flex items-center justify-between">
								<span class="text-xs font-bold tracking-widest text-slate-400 uppercase">
									{baseInfo[base].label}
								</span>
								<button
									onclick={() => copyToClipboard(calcResults[base], 'calc-' + base)}
									disabled={!calcResults[base]}
									class="flex items-center gap-1.5 rounded-full px-3 py-1.5 text-xs font-bold transition-all {copiedKey ===
									'calc-' + base
										? 'bg-emerald-500 text-white shadow-lg shadow-emerald-200'
										: 'bg-slate-100 text-slate-600 hover:bg-slate-200'}"
								>
									{#if copiedKey === 'calc-' + base}
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
										복사하기
									{/if}
								</button>
							</div>
							<div class="font-mono text-lg break-all text-slate-800">
								{calcResults[base] || '—'}
							</div>
						</div>
					{/each}
				</div>
			</section>

			<!-- Guide / Tips -->
			<div class="mt-8 rounded-3xl border border-slate-100 bg-white p-6 shadow-sm">
				<h3 class="mb-3 flex items-center gap-2 font-bold text-slate-800">
					<span class="text-indigo-500">💡</span> 기능 안내
				</h3>
				<div class="grid gap-6 md:grid-cols-2">
					<ul class="space-y-2 text-sm leading-relaxed text-slate-500">
						<li>
							• <strong>실시간 변환:</strong> 입력과 동시에 HEX·DEC·BIN·OCT 결과가 갱신됩니다.
						</li>
						<li>
							• <strong>혼합 진수 연산:</strong> 접두어를 붙이면 다른 진수 리터럴을 섞어 쓸 수 있습니다.
						</li>
						<li>
							• <strong>콤마 자동 제거:</strong> 콤마가 포함된 숫자를 붙여넣으면 콤마가 자동으로 제거됩니다.
						</li>
					</ul>
					<ul class="space-y-2 text-sm leading-relaxed text-slate-500">
						<li>
							• <strong>BigInteger 정수 연산:</strong> 오버플로우 없는 정수 계산을 지원합니다 (나눗셈은
							몫 계산).
						</li>
						<li>
							• <strong>안전한 처리:</strong> 모든 계산은 브라우저 내에서만 처리되며 서버로 전송되지 않습니다.
						</li>
					</ul>
				</div>
			</div>
		</div>
	</main>

	<footer class="mt-auto py-10 text-center text-xs text-slate-400">
		© 2026 Klutter HEX Calculator Tool.
	</footer>
</div>
