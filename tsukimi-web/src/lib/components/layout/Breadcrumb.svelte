<script lang="ts">
	import { page } from '$app/state';
	import { capitalCase } from 'change-case';

	// use svelte i18n
	let parts = $derived(['home', ...page.url.pathname.split('/').filter(Boolean)]);
</script>

<nav class="flex items-center gap-1 text-sm">
	{#each parts as part, i (i)}
		{@const is_last = i === parts.length - 1}
		<a
			href={is_last ? '#' : '/' + parts.slice(1, i + 1).join('/')}
			class={[!is_last && 'hover:underline']}
			aria-disabled={is_last}
			class:font-bold={is_last}
			>{capitalCase(part)}
		</a>

		{#if i < parts.length - 1}
			<span class="mgc_right_line"></span>
		{/if}
	{/each}
</nav>
