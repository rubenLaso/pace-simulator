import React from "react";
import 'react-tabs/style/react-tabs.css';
import { Container } from "@mui/system";

import { NumField } from "../../components/utils/NumField";

import { set_laptime_cold_track_empty_tank, set_laptime_cold_track_full_tank, set_laptime_cold_track_old_tyres, set_laptime_warm_track_full_tank } from 'glc-wasm';

export const LaptimesPane = () => {
	return (
		<Container>
			<NumField id="laptime_cold_track_empty_tank" label="Cold track (empty tank)"
				onupdate={(value) => {
					if (!isNaN(value)) {
						console.log("Laptime cold track (empty tank): ", value);
						set_laptime_cold_track_empty_tank(value);
					}
				}}
			/>
			<NumField id="laptime_cold_track_full_tank" label="Cold track (full tank)"
				onupdate={(value) => {
					if (!isNaN(value)) {
						console.log("Laptime cold track (full tank): ", value);
						set_laptime_cold_track_full_tank(value);
					}
				}}
			/>
			<NumField id="laptime_warm_track_full_tank" label="Warm track (full tank)"
				onupdate={(value) => {
					if (!isNaN(value)) {
						console.log("Laptime warm track (full tank): ", value);
						set_laptime_warm_track_full_tank(value);
					}
				}}
			/>
			<NumField id="laptime_cold_track_old_tyres" label="Cold track (old tyres)"
				onupdate={(value) => {
					if (!isNaN(value)) {
						console.log("Laptime cold track (old tyres): ", value);
						set_laptime_cold_track_old_tyres(value);
					}
				}}
			/>
		</Container>
	)
}