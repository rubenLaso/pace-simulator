import React from "react";
import 'react-tabs/style/react-tabs.css';
import { Container, Box } from "@mui/system";

import { NumField } from "../../components/utils/NumField";

export const RacePane = () => {
	return (
		<Container>
			<Box sx={{ display: 'flex', flexDirection: 'row', flexWrap: 'wrap' }}>
				<NumField id="race_length_hours" label="Race length (hours)"
					onupdate={(value) => {
						if (!isNaN(value)) {
							console.log("Race length (hours): ", value);
						}
					}}
				/>
				<NumField id="race_length_mins" label="Race length (mins)"
					onupdate={(value) => {
						if (!isNaN(value)) {
							console.log("Race length (mins): ", value);
						}
					}}
				/>
			</Box>

			<Box sx={{ display: 'flex', flexDirection: 'row', flexWrap: 'wrap' }}>
				<NumField id="race_start_hours" label="Race start (hours)"
					onupdate={(value) => {
						if (!isNaN(value)) {
							console.log("Race start (hours): ", value);
						}
					}}
				/>
				<NumField id="race_start_mins" label="Race start (mins)"
					onupdate={(value) => {
						if (!isNaN(value)) {
							console.log("Race length (mins): ", value);
						}
					}}
				/>
			</Box>
			<Box sx={{ display: 'flex', flexDirection: 'row', flexWrap: 'wrap' }}>
				<NumField id="grid_size" label="Grid size"
					onupdate={(value) => {
						if (!isNaN(value)) {
							console.log("Grid size: ", value);
						}
					}}
				/>
				<NumField id="simulation_timestep" label="Simulation timestep (secs)"
					onupdate={(value) => {
						if (!isNaN(value)) {
							console.log("Simulation timestep (secs): ", value);
						}
					}}
				/>
			</Box>
		</Container>
	)
}