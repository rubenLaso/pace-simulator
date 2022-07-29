import React from "react";
// import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import 'react-tabs/style/react-tabs.css';
import { Container } from "@mui/system";
import { Button } from '@mui/material';
import PropTypes from 'prop-types';
import Tabs from '@mui/material/Tabs';
import Tab from '@mui/material/Tab';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';

import { RacePane } from "../components/Race/RacePane";
import { LaptimesPane } from "../components/Car/LaptimesPane";
import { CarInfoPane } from "../components/Car/CarInfoPane";
import { TrackTempLineChart } from "../components/Track/TrackTempLineChart";
import { TyrePerfLineChart } from "../components/Tyres/TyrePerfLineChart";

function TabPanel(props) {
	const { children, value, index, ...other } = props;

	return (
		<div
			role="tabpanel"
			hidden={value !== index}
			id={`simple-tabpanel-${index}`}
			aria-labelledby={`simple-tab-${index}`}
			{...other}
		>
			<Box sx={{ p: 3 }}>
				<Typography component={'span'}>{children}</Typography>
			</Box>
		</div>
	);
}

TabPanel.propTypes = {
	children: PropTypes.node,
	index: PropTypes.number.isRequired,
	value: PropTypes.number.isRequired,
};

function a11yProps(index) {
	return {
		id: `simple-tab-${index}`,
		'aria-controls': `simple-tabpanel-${index}`,
	};
}

export const MyTabs = () => {
	const [value, setValue] = React.useState(0);

	const handleChange = (_event, newValue) => {
		setValue(newValue);
	};

	function handleSubmit(e) {
		console.log("You clicked \"Run\"!!!");
		return true;
	}

	return (
		<Box sx={{ width: '100%' }}>
			<Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
				<Tabs value={value} onChange={handleChange} >
					<Tab label="Race Info" {...a11yProps(0)} />
					<Tab label="Reference laptimes" {...a11yProps(1)} />
					<Tab label="Car Info" {...a11yProps(2)} />
					<Tab label="Track Info" {...a11yProps(3)} />
					<Tab label="Tyres Info" {...a11yProps(4)} />
					<Tab label="Simulate" {...a11yProps(5)} />
				</Tabs>
			</Box>
			<TabPanel value={value} index={0}>
				<RacePane />
			</TabPanel>
			<TabPanel value={value} index={1}>
				<LaptimesPane />
			</TabPanel>
			<TabPanel value={value} index={2}>
				<CarInfoPane />
			</TabPanel>
			<TabPanel value={value} index={3}>
				<TrackTempLineChart />
			</TabPanel>
			<TabPanel value={value} index={4}>
				<TyrePerfLineChart />
			</TabPanel>
			<TabPanel value={value} index={5}>
				<Button variant="contained" onClick={handleSubmit}>
					Run simulation
				</Button>
			</TabPanel>
		</Box>
	)
}

export default function App() {
	return (
		<React.Fragment>
			<MyTabs />
		</React.Fragment >
	);
}