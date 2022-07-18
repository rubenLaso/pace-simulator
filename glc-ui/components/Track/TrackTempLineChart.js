import React from "react"
import { useState } from "react";
import { Container } from "@mui/material";
import { add_track_temp, set_track_temp, build_track_temp_spline, expected_laptime_by_track_temp, get_laptime_warm_track_full_tank, get_laptime_cold_track_full_tank, get_max_track_temp, get_min_track_temp } from 'glc-wasm';

import { Line } from "react-chartjs-2";
import dragdataPlugin from "chartjs-plugin-dragdata";

import { Chart, registerables } from "chart.js";
Chart.register(...registerables, dragdataPlugin);

let isDraggingPoint;

const tyres_default_xs = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23];
const tyres_default_ys = [21, 20, 19, 18, 17, 17, 18, 19, 21, 23, 26, 30, 34, 39, 42, 41, 38, 35, 31, 26, 24, 23, 22, 21];

export const TrackTempLineChart = ({ xs = tyres_default_xs, ys = tyres_default_ys, ymax = 60, ymin = 10, xmax = 23, xmin = 0, title = 'Temperature' }) => {
	for (let i = 0; i < xs.length; i++) {
		const x = xs[i];
		const y = ys[i];
		add_track_temp(x, y);
	}
	build_track_temp_spline();

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
			},
			x: {
				type: 'linear',
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
					set_track_temp(index, y);
					build_track_temp_spline();
				}
			},
			tooltip: {
				callbacks: {
					title: function (tooltipItem, _data) {
						const x = tooltipItem[0].label;
						const temp = new Intl.NumberFormat('en-UK', { style: 'unit', unit: 'hour' }).format(x);

						return 'Track temperature: ' + temp;
					},
					label: function (context) {
						let label = context.dataset.label || '';

						if (label) {
							label += ': ';
						}
						if (context.parsed.y !== null) {
							label += new Intl.NumberFormat('en-UK', { style: 'unit', unit: 'celsius' }).format(context.parsed.y);
						}

						return label;
					},
					afterLabel: function (context) {
						const track_temp = context.parsed.y;

						let laptime = 0.0;

						if (track_temp >= get_max_track_temp()) {
							laptime = get_laptime_warm_track_full_tank();
						} else if (track_temp <= get_min_track_temp()) {
							laptime = get_laptime_cold_track_full_tank();
						} else {
							laptime = expected_laptime_by_track_temp(track_temp);
						}

						const time = new Intl.NumberFormat('en-UK', { style: 'unit', unit: 'second' }).format(laptime);

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