import React from "react"
import { useState } from "react";
import { Container } from "@mui/material";
import { add_coords, set_coords, build_spline, expected_laptime } from 'glc-wasm';

import { Line } from "react-chartjs-2";
import dragdataPlugin from "chartjs-plugin-dragdata";

import { Chart, registerables } from "chart.js";
Chart.register(...registerables, dragdataPlugin);

let isDraggingPoint;

export const DraggableLineChart = ({ xs, ys }) => {
	for (let i = 0; i < xs.length; i++) {
		const x = xs[i];
		const y = ys[i];
		add_coords(x, y);
	}

	const options = {
		hover: {
			mode: "nearest",
		},
		datasets: {
			cubicInterpolationMode: 'monotone',
			scatter: {
				borderWidth: 2.5,
				fill: false,
				pointRadius: 10,
				pointHitRadius: 15,
				showLine: true
			}
		},
		scales: {
			y: {
				min: 0,
				max: 100
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
					set_coords(index, value);
					build_spline();
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
						let label = context.dataset.label || '';
						const y = context.parsed.y;
						const time = new Intl.NumberFormat('en-UK', { style: 'unit', unit: 'second' }).format(expected_laptime(y));
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
				label: "Performance",
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