import React from "react";
import CssBaseline from '@mui/material/CssBaseline';
import { useState } from "react";
import { Container } from "@mui/material";
import { greet, add_coords, print_all_coords, new_coords, set_coords } from 'glc-wasm';

import { Line } from "react-chartjs-2";
import dragdataPlugin from "chartjs-plugin-dragdata";

import { Chart, registerables } from "chart.js";
Chart.register(...registerables, dragdataPlugin);

let isDraggingPoint;

const xs = [100, 90, 80, 70, 60, 50, 40, 30, 20, 10, 0];
const ys = [100, 96, 94, 91, 89, 87, 83, 76, 67, 55, 40];

new_coords();

for (let i = 0; i < xs.length; i++) {
	const x = xs[i];
	const y = ys[i];
	add_coords(x, y);
}

print_all_coords();

export const options = {
	hover: {
		mode: "dataset"
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
				print_all_coords();
			}
		},
	},
};

export default function App() {
	greet()

	const [userData, setUserData] = useState({
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
		<React.Fragment>
			<CssBaseline>
				<Container>
					<script src="https://cdn.jsdelivr.net/npm/chartjs-plugin-dragdata@2.2.3/dist/chartjs-plugin-dragdata.min.js"></script>
					<Line options={options} data={userData} />
				</Container>
			</CssBaseline>
		</React.Fragment>
	);
}