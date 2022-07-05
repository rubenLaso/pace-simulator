import React from "react";
import CssBaseline from '@mui/material/CssBaseline';
import { useState } from "react";
import { Container } from "@mui/material";
import { greet } from 'glc-wasm';

import { Line } from "react-chartjs-2";
import dragdataPlugin from "chartjs-plugin-dragdata";

import { Chart, registerables } from "chart.js";
Chart.register(...registerables, dragdataPlugin);

greet();

let isDraggingPoint;

export const options = {
	hover: {
		mode: "dataset"
	},
	datasets: {
		scatter: {
			borderWidth: 2.5,
			fill: false,
			pointRadius: 10,
			pointHitRadius: 15,
			showLine: true
		}
	},
	layout: {
		padding: {
			left: 20,
			right: 20,
			top: 20,
			bottom: 10
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
			round: 1,
			showTooltip: true,
			onDragStart: (e, datasetIndex, index, value) => {
				isDraggingPoint = true;
			},
			onDragEnd: (e, datasetIndex, index, value) => {
				isDraggingPoint = false;
			}
		}
	}
};

export default function App() {
	greet()

	const [userData, setUserData] = useState({
		labels: [100, 75, 50, 25, 0],
		datasets: [
			{
				label: "Performance",
				data: [100, 90, 80, 60, 20],
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
					Hello World!!!
					<script src="https://cdn.jsdelivr.net/npm/chartjs-plugin-dragdata@2.2.3/dist/chartjs-plugin-dragdata.min.js"></script>
					<Line options={options} data={userData} />
				</Container>
			</CssBaseline>
		</React.Fragment>
	);
}