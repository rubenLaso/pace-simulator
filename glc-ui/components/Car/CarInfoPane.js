import React from "react";
import 'react-tabs/style/react-tabs.css';
import { Container, Box } from "@mui/system";

import { NumField } from "../../components/utils/NumField";

export const CarInfoPane = () => {
	return (
		<Container>
			<Box sx={{ display: 'flex', flexDirection: 'row', flexWrap: 'wrap' }}>
				<NumField id="tank_capacity" label="Tank capacity (L)"
					onupdate={(value) => {
						if (!isNaN(value)) {
							console.log("Tank capacity (L): ", value);
						}
					}}
				/>
				<NumField id="fuel_per_lap" label="Fuel per lap (L)"
					onupdate={(value) => {
						if (!isNaN(value)) {
							console.log("Fuel per lap (L): ", value);
						}
					}}
				/>
				<NumField id="tyres_critical_state" label="Tyres critical state (%)"
					onupdate={(value) => {
						if (!isNaN(value)) {
							console.log("Tyres critical state (%): ", value);
						}
					}}
				/>
			</Box>
			<Box sx={{ display: 'flex', flexDirection: 'row', flexWrap: 'wrap' }}>
				<NumField id="time_to_fill_tank" label="Time to fill tank (s)"
					onupdate={(value) => {
						if (!isNaN(value)) {
							console.log("Time to fill tank (s): ", value);
						}
					}}
				/>
				<NumField id="time_to_change_tyres" label="Time to change tyres (s)"
					onupdate={(value) => {
						if (!isNaN(value)) {
							console.log("Time to change tyres (s): ", value);
						}
					}}
				/>
				<NumField id="time_to_drive_through" label="Time to drive through pits (s)"
					onupdate={(value) => {
						if (!isNaN(value)) {
							console.log("Time to drive through pits (s): ", value);
						}
					}}
				/>
			</Box>
		</Container>
	)
}