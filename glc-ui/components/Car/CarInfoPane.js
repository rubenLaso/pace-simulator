import React from "react";
import 'react-tabs/style/react-tabs.css';
import { Container, Box } from "@mui/system";

import { NumField } from "../../components/utils/NumField";

import { set_fuel_per_lap, set_fuel_tank_capacity, set_time_to_change_tyres, set_time_to_drive_through, set_time_to_fill_fuel_tank, default_tank_capacity, default_fuel_per_lap, default_time_to_fill_tank, default_time_to_change_tyres, default_time_to_drive_through } from 'glc-wasm';

export const CarInfoPane = () => {
	return (
		<Container>
			<Box sx={{ display: 'flex', flexDirection: 'row', flexWrap: 'wrap' }}>
				<NumField id="tank_capacity" label="Tank capacity (L)" default_value={default_tank_capacity()}
					onupdate={(value) => {
						if (!isNaN(value)) {
							console.log("Tank capacity (L): ", value);
							set_fuel_tank_capacity(value);
						}
					}}
				/>
				<NumField id="fuel_per_lap" label="Fuel per lap (L)" default_value={default_fuel_per_lap()}
					onupdate={(value) => {
						if (!isNaN(value)) {
							console.log("Fuel per lap (L): ", value);
							set_fuel_per_lap(value);
						}
					}}
				/>
				<NumField id="tyres_critical_state" label="Tyres critical state (%)" default_value={30.0}
					onupdate={(value) => {
						if (!isNaN(value)) {
							console.log("Tyres critical state (%): ", value);
							// TODO: insert function
						}
					}}
				/>
			</Box>
			<Box sx={{ display: 'flex', flexDirection: 'row', flexWrap: 'wrap' }}>
				<NumField id="time_to_fill_tank" label="Time to fill tank (s)" default_value={default_time_to_fill_tank()}
					onupdate={(value) => {
						if (!isNaN(value)) {
							console.log("Time to fill tank (s): ", value);
							set_time_to_fill_fuel_tank(value);
						}
					}}
				/>
				<NumField id="time_to_change_tyres" label="Time to change tyres (s)" default_value={default_time_to_change_tyres()}
					onupdate={(value) => {
						if (!isNaN(value)) {
							console.log("Time to change tyres (s): ", value);
							set_time_to_change_tyres(value);
						}
					}}
				/>
				<NumField id="time_to_drive_through" label="Time to drive through pits (s)" default_value={default_time_to_drive_through()}
					onupdate={(value) => {
						if (!isNaN(value)) {
							console.log("Time to drive through pits (s): ", value);
							set_time_to_drive_through(value);
						}
					}}
				/>
			</Box>
		</Container>
	)
}