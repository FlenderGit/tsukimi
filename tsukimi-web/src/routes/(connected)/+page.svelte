<script lang="ts">
	import type { Extension } from '$lib/types';
	import { kebabCase } from 'change-case';

	const extensions: Extension[] = [
		{
			name: 'HanaScripter',
			description: 'This is an example extension.',
			current_version: '1.0.0'
		},
		{
			name: 'SRPG Studio',
			description: 'This extension does something else.',
			current_version: '2.3.4'
		}
	];
</script>

<input type="search" placeholder="Search extensions..." class="input" />

<button class="btn">1</button>
<button class="btn size-8">2</button>
<button class="btn" disabled>Jean</button>
<table>
	<thead>
		<tr>
			<th>Name</th>
			<th>Description</th>
			<th>Current version</th>
		</tr>
	</thead>
	<tbody>
		{#each extensions as extension (extension.name)}
			{@const url = '/engines/' + kebabCase(extension.name)}
			<tr>
				<td>
					<a href={url} class="link">
						{extension.name}
					</a>
				</td>
				<td>{extension.description}</td>
				<td>
					<a href="{url}#current-version" class="link">
						{extension.current_version}
					</a>
				</td>
			</tr>
		{/each}
	</tbody>
</table>
