import React from "react";
import { greet } from 'glc-wasm';

import { DraggableLineChart } from "../components/DraggableLineChart";


export default function App() {
	greet()

	const xs = [100, 90, 80, 70, 60, 50, 40, 30, 20, 10, 0];
	const ys = [100, 96, 94, 91, 89, 87, 83, 76, 67, 55, 40];

	return (
		<React.Fragment>
			<DraggableLineChart xs={xs} ys={ys} />
		</React.Fragment>
	);
}