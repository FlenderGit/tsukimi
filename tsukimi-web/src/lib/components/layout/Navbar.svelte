<script lang="ts">
	import { page } from '$app/state';
	import logo from '$lib/assets/logo.png';

	type Link = {
		name: string;
		href: string;
		icon?: string;
	};

	const LINKS: Link[] = [
		{ name: 'Home', href: '/', icon: 'home_3_line' },
		{ name: 'Engines', href: '/engines', icon: 'engine_line' },
		{ name: 'Visual Novels', href: '/visual-novels', icon: 'book_2_line' }
	];
</script>

<aside class="flex w-64 flex-col border-r-2 border-r-border bg-background-alt">
	<header class="flex items-center gap-2 p-2">
		<img src={logo} alt="Tsukimi Logo" width="48" height="48" class="rounded-lg" />
		<h1 class="text-center text-2xl font-bold">Tsukimi</h1>
	</header>

	<hr />

	<nav class="flex-1 overflow-y-auto text-base">
		<ul class="flex flex-col">
			{#each LINKS as link (link.href)}
				{@const is_current = page.url.pathname === link.href}
				<li>
					<a
						href={link.href}
						class="flex items-center gap-4 px-2 py-1 hover:bg-gray-100"
						aria-current={is_current ? 'page' : undefined}
						class:font-bold={is_current}
						title={link.name}
						aria-label={link.name}
					>
						{#if link.icon}
							<span class="mgc_{link.icon}"></span>
						{/if}
						{link.name}
					</a>
				</li>
			{/each}
		</ul>
	</nav>

	<footer>
		<p>Flender</p>
	</footer>
</aside>
