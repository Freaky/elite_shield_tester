<script>
  import Nav from '../components/Nav.svelte';
</script>

<style>
	header {
		display: flex;
		justify-content: space-between;
	}
</style>

<header>
  <h1 id="Head"><a href=""><strong>Elite Dangerous</strong> <span>Shield Loadout Optimiser</span></a></h1>
  <Nav />
</header>

<main>
  <slot></slot>
</main>
