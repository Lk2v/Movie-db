<script lang="ts">
	import type { Movie } from '$api/movie';
	import type { MovieGenreCountStats } from '$api/stats';
	import { Chart } from 'chart.js/auto';
	import { onMount } from 'svelte';

	let {
		data
	}: {
		data: MovieGenreCountStats;
	} = $props();

	onMount(() => {
		init();
	});

	$effect(() => {
		update(data);
	});

	let canvas: HTMLCanvasElement;
	let chart: Chart;

	function init() {
		const ctx = canvas?.getContext('2d');

		if (ctx) {
			const { labels, values } = parseData(data);

			chart = new Chart(ctx, {
				type: 'bar',

				data: {
					labels: labels,
					datasets: [
						{
							label: 'Movies count',
							data: values,
							backgroundColor: 'rgba(54, 162, 235, 0.2)',
							borderColor: 'rgba(54, 162, 235, 0.6)',
							borderWidth: 1
						}
					]
				},
				options: {
					responsive: true,
					maintainAspectRatio: false,
					scales: {
                        x: {
                            grid: {
                                display: false
                            }
                        },
						y: {
							beginAtZero: true,
                            grid: {
                                display: false
                            }
						}
					},
				}
			});
		}
	}

	function parseData(d: MovieGenreCountStats) {
		const labels = d.map((item) => item.genre_name);
		const values = d.map((item) => item.genre_count);

		return { labels, values };
	}

	function update(d: MovieGenreCountStats) {
        if (chart) {
			console.log('updating chart');
            const { labels, values } = parseData(d);

			chart.data.labels = labels;
            chart.data.datasets[0].data = values;
            chart.update();
        }
    }
</script>

<div class="graph-cell">
	<canvas bind:this={canvas}></canvas>
</div>

<style>
	.graph-cell {
		width: 100%;
		height: 100%;

		padding: var(--cell-padding);
		box-sizing: border-box;

		display: flex;
		flex-direction: column;
	}
</style>
