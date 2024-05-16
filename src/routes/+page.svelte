<script>
	import Greet from '../lib/Greet.svelte';

	/** @type {import('./$types').PageData} */
	export let data;
</script>

<div class="my-8 bg-white w-full p-8 bg-opacity-35 select-none" data-tauri-drag-region="self">
	hold and drag me to move the window
</div>

<div class="m-4">
	<h1>SvelteKit + TailwindCSS + Tauri + Fastapi</h1>
	<Greet />

	{data.data.message}
	<!-- tauri takes a bit to start the python api, reloading may be needed, (no fetches on load?, find smth maybe) -->
</div>
