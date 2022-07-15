import { Container } from "@mui/system";
import React from "react";
import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import 'react-tabs/style/react-tabs.css';

import { TyrePerfLineChart } from "../components/Tyres/TyrePerfLineChart";
import { TrackTempLineChart } from "../components/Track/TrackTempLineChart";

export const MyTabs = () => {
	return (
		<Container>
			<script src="https://unpkg.com/react-tabs/dist/react-tabs.production.min.js" />
			<Tabs>
				<TabList>
					<Tab>Track</Tab>
					<Tab>Tyres</Tab>
				</TabList>

				<TabPanel>
					<TrackTempLineChart />
				</TabPanel>
				<TabPanel>
					<TyrePerfLineChart />
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