<script lang="ts">
	import { resolve } from '$app/paths';
	import { ToolStatus, type Tool } from '$lib/types';

	const statusLabels: Record<ToolStatus, string> = {
		[ToolStatus.READY]: '준비됨',
		[ToolStatus.COMING_SOON]: '준비 중',
		[ToolStatus.DRAFT]: '비공개'
	};

	const tools: Tool[] = [
		{
			id: 'luxury',
			title: 'ℒ𝓊𝓍𝓊𝓇𝓎',
			description: '입력한 영문 텍스트를 멋진 필기체나 손글씨 스타일로 변환합니다.',
			icon: '✍️',
			color: 'bg-amber-100 text-amber-600',
			status: ToolStatus.READY
		},
		{
			id: 'json-viewer',
			title: 'JSON 포매터',
			description: '복잡하고 읽기 어려운 JSON 데이터를 깔끔하게 정리하고 시각화합니다.',
			icon: '{}',
			color: 'bg-blue-100 text-blue-600',
			status: ToolStatus.COMING_SOON
		},
		{
			id: 'unit-calc',
			title: '단위 변환 마스터',
			description: '다양한 측정 단위 간의 변환을 쉽고 빠르게 도와줍니다.',
			icon: '📏',
			color: 'bg-emerald-100 text-emerald-600',
			status: ToolStatus.DRAFT
		}
	];

	let searchQuery = $state('');
	let filteredTools = $derived(
		tools
			.filter((tool) => tool.status !== ToolStatus.DRAFT)
			.filter(
				(tool) =>
					tool.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
					tool.description.toLowerCase().includes(searchQuery.toLowerCase())
			)
	);
</script>

<div class="min-h-screen bg-[#f8fafc] font-sans text-slate-900">
	<!-- Header -->
	<header class="sticky top-0 z-20 border-b border-slate-200 bg-white">
		<div class="mx-auto flex max-w-6xl items-center justify-between px-6 py-4">
			<div class="gap-3transition-opacity flex items-center">
				<div class="text-2xl">📦</div>
				<h1 class="text-xl font-black tracking-tight text-slate-800 uppercase">Klutter</h1>
			</div>
			<nav class="flex items-center gap-4">
				<a
					href="https://github.com/4BMtolobPL/klutter"
					class="rounded-full p-2 text-slate-500 transition-colors hover:bg-slate-100"
					aria-label="GitHub Repository"
					target="_blank"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="20"
						height="20"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
						><path
							d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"
						></path><path d="M9 18c-4.51 2-5-2-7-2"></path></svg
					>
				</a>
			</nav>
		</div>
	</header>

	<main class="mx-auto max-w-6xl px-6 py-12">
		<!-- Hero / Intro -->
		<div class="mb-12">
			<h2 class="mb-4 text-4xl font-extrabold text-slate-900">소소하고 유용한 도구들.</h2>
			<p class="max-w-2xl text-lg text-slate-500">
				Axum과 SvelteKit으로 만든 "잡다구리" 웹 도구 모음입니다.
			</p>
		</div>

		<!-- Search Bar -->
		<div class="relative mb-10">
			<div class="pointer-events-none absolute inset-y-0 left-4 flex items-center">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-5 w-5 text-slate-400"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
					/>
				</svg>
			</div>
			<input
				type="text"
				bind:value={searchQuery}
				placeholder="도구 검색 (예: 필기체, 계산기...)"
				class="w-full rounded-2xl border border-slate-200 bg-white py-4 pr-4 pl-12 shadow-sm transition-all focus:border-transparent focus:ring-2 focus:ring-indigo-500 focus:outline-none"
			/>
		</div>

		<!-- Tools Grid -->
		<div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
			{#each filteredTools as tool (tool.id)}
				<a
					href={tool.status === ToolStatus.READY
						? resolve('/tools/[slug]', { slug: tool.id })
						: 'javascript:void(0)'}
					class="group relative rounded-3xl border border-slate-200 bg-white p-6 transition-all hover:-translate-y-1 hover:shadow-xl hover:shadow-slate-200/50 {tool.status !==
					ToolStatus.READY
						? 'cursor-not-allowed opacity-75'
						: ''}"
				>
					<div class="mb-6 flex items-start justify-between">
						<div
							class="h-12 w-12 {tool.color} flex items-center justify-center rounded-2xl text-2xl shadow-inner"
						>
							{tool.icon}
						</div>
						<span
							class="rounded-md px-2 py-1 text-[10px] font-bold tracking-widest uppercase {tool.status ===
							ToolStatus.READY
								? 'bg-indigo-50 text-indigo-600'
								: 'bg-slate-100 text-slate-400'}"
						>
							{statusLabels[tool.status]}
						</span>
					</div>

					<h3
						class="mb-2 text-xl font-bold text-slate-800 transition-colors group-hover:text-indigo-600"
					>
						{tool.title}
					</h3>
					<p class="text-sm leading-relaxed text-slate-500">
						{tool.description}
					</p>

					{#if tool.status === ToolStatus.READY}
						<div
							class="mt-6 flex items-center text-sm font-bold text-indigo-600 opacity-0 transition-opacity group-hover:opacity-100"
						>
							도구 열기
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="ml-1 h-4 w-4"
								fill="none"
								viewBox="0 0 24 24"
								stroke="currentColor"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M9 5l7 7-7 7"
								/>
							</svg>
						</div>
					{/if}
				</a>
			{/each}

			<!-- Empty State -->
			{#if filteredTools.length === 0}
				<div class="col-span-full py-20 text-center">
					<div class="mb-4 text-4xl">🔍</div>
					<h3 class="text-xl font-bold text-slate-800">검색 결과가 없습니다</h3>
					<p class="text-slate-500">다른 검색어를 입력하거나 새로운 도구를 제안해 보세요!</p>
				</div>
			{/if}
		</div>
	</main>

	<footer class="mt-20 border-t border-slate-200 py-10">
		<div
			class="mx-auto flex max-w-6xl flex-col items-center justify-between gap-4 px-6 text-sm text-slate-400 md:flex-row"
		>
			<p>© 2026 Klutter. 소소한 도구들의 모음.</p>
			<!-- <div class="flex items-center gap-6">
				<a href={resolve('/')} class="transition-colors hover:text-slate-600">아이디어 제안</a>
				<a href={resolve('/')} class="transition-colors hover:text-slate-600">정보</a>
			</div> -->
		</div>
	</footer>
</div>
