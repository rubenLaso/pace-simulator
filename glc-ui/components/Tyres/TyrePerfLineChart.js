import React from "react"
import { useState } from "react";
import { Container } from "@mui/material";
import { add_tyre_wear, set_tyre_wear, build_tyre_wear_spline, expected_laptime } from 'glc-wasm';

import { Line } from "react-chartjs-2";
import dragdataPlugin from "chartjs-plugin-dragdata";

import { Chart, registerables } from "chart.js";
Chart.register(...registerables, dragdataPlugin);

let isDraggingPoint;

const tyres_default_xs = [100, 95, 90, 80, 70, 60, 50, 40, 20, 0];
const tyres_default_ys = [100, 97, 95, 94, 92, 90, 87, 82, 72, 60];

export const TyrePerfLineChart = ({ xs = tyres_default_xs, ys = tyres_default_ys, ymax = 100, ymin = 0, xmax = 100, xmin = 0, title = 'Performance' }) => {
	for (let i = 0; i < xs.length; i++) {
		const x = xs[i];
		const y = ys[i];
		add_tyre_wear(x, y);
	}
	build_tyre_wear_spline();

	const options = {
		hover: {
			mode: "nearest",
		},
		datasets: {
			cubicInterpolationMode: 'monotone',
		},
		scales: {
			y: {
				type: 'linear',
				min: ymin,
				max: ymax,
				ticks: {
					display: false
				}
			},
			x: {
				type: 'linear',
				reverse: 'true',
				min: xmin,
				max: xmax
			}
		},
		plugins: {
			dragData: {
				round: 2,
				showTooltip: true,
				onDragStart: (_e, _datasetIndex, _index, _value) => {
					isDraggingPoint = true;
				},
				onDragEnd: (_e, _datasetIndex, index, value) => {
					isDraggingPoint = false;
					const x = xs[index];
					const y = value;
					set_tyre_wear(index, y);
					build_tyre_wear_spline();
				}
			},
			tooltip: {
				callbacks: {
					title: function (tooltipItem, _data) {
						const x = tooltipItem[0].label;

						return 'Tyre state: ' + x + '%';
					},
					label: function (context) {
						let label = context.dataset.label || '';

						if (label) {
							label += ': ';
						}
						if (context.parsed.y !== null) {
							label += new Intl.NumberFormat('en-UK').format(context.parsed.y) + '%';
						}

						return label;
					},
					afterLabel: function (context) {
						const tyre_state = context.parsed.y;
						const time = new Intl.NumberFormat('en-UK', { style: 'unit', unit: 'second' }).format(expected_laptime(tyre_state));

						return 'Laptime: ' + time;
					}
				}
			}
		}
	};

	const [userData, _setUserData] = useState({
		labels: xs,
		datasets: [
			{
				label: title,
				data: ys,
				tension: 0.2,
				borderColor: "black",
				borderWidth: 2,
			},
		],
	});

	return (
		<Container>
			<script src="https://cdn.jsdelivr.net/npm/chartjs-plugin-dragdata@2.2.3/dist/chartjs-plugin-dragdata.min.js"></script>
			<Line options={options} data={userData} />
		</Container>
	);
}