<script lang="ts">
	import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
	import Navbar from '$lib/components/layout/Navbar.svelte';

	let { children } = $props();
</script>

<div class="flex flex-1">
	<Navbar />
	<main class="@container mx-auto max-w-5xl flex-1 p-4">
		<Breadcrumb />
		{@render children?.()}
	</main>
</div>
