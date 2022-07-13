import { Container } from "@mui/system";
import React from "react";
import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import 'react-tabs/style/react-tabs.css';

import { DraggableLineChart } from "../components/DraggableLineChart";

export const MyTabs = () => {
	const xs = [100, 90, 80, 70, 60, 50, 40, 30, 20, 10, 0];
	const ys = [100, 96, 94, 91, 89, 87, 83, 76, 67, 55, 40];

	return (
		<Container>
			<script src="https://unpkg.com/react-tabs/dist/react-tabs.production.min.js" />
			<Tabs>
				<TabList>
					<Tab>Tyres</Tab>
				</TabList>

				<TabPanel>
					<DraggableLineChart xs={xs} ys={ys} />
				</TabPanel>
			</Tabs>
		</Container>
	)
}

export default function App() {
	return (
		<React.Fragment>
			<MyTabs />
		</React.Fragment>
	);
}