<script>
	export var value;
	export var name;
	export var min;
	export var max;

	$: if (value < min || value > max) value = max;
</script>

<input type="number" bind:value><input type="range" {name} {min} {max} bind:value>
