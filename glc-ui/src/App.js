import React from "react";
import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import 'react-tabs/style/react-tabs.css';
import { Container } from "@mui/system";

import { TyrePerfLineChart } from "../components/Tyres/TyrePerfLineChart";
import { TrackTempLineChart } from "../components/Track/TrackTempLineChart";
import { NumField } from "../components/utils/NumField";

import { set_laptime_cold_track_full_tank, set_laptime_warm_track_full_tank } from 'glc-wasm';

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
					<NumField id="laptime_cold_track" label="Cold track laptime"
						onupdate={(value) => {
							if (!isNaN(value)) {
								set_laptime_cold_track_full_tank(value);
							}
						}} />
					<NumField id="laptime_warm_track" label="Warm track laptime"
						onupdate={(value) => {
							if (!isNaN(value)) {
								set_laptime_warm_track_full_tank(value);
							}
						}} />
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