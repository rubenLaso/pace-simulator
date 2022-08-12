import React from "react";
import 'react-tabs/style/react-tabs.css';
import { Container } from "@mui/system";

import { NumField } from "../../components/utils/NumField";

import { set_laptime_cold_track_empty_tank, set_laptime_cold_track_full_tank, set_laptime_cold_track_old_tyres, set_laptime_warm_track_full_tank, default_laptime_cold_track_empty_tank, default_laptime_cold_track_full_tank, default_laptime_warm_track_full_tank, default_laptime_cold_track_old_tyres } from 'glc-wasm';

export const LaptimesPane = () => {
	let default_cold_track_empty_tank = default_laptime_cold_track_empty_tank();
	let default_cold_track_full_tank = default_laptime_warm_track_full_tank();
	let default_warm_track_full_tank = default_laptime_warm_track_full_tank();
	let default_cold_track_old_tyres = default_laptime_cold_track_old_tyres();

	return (
		<Container>
			<NumField id="laptime_cold_track_empty_tank" label="Cold track (empty tank)" default_value={default_cold_track_empty_tank}
				onupdate={(value) => {
					if (!isNaN(value)) {
						console.log("Laptime cold track (empty tank): ", value);
						set_laptime_cold_track_empty_tank(value);
					}
				}}
			/>
			<NumField id="laptime_cold_track_full_tank" label="Cold track (full tank)" default_value={default_warm_track_full_tank}
				onupdate={(value) => {
					if (!isNaN(value)) {
						console.log("Laptime cold track (full tank): ", value);
						set_laptime_cold_track_full_tank(value);
					}
				}}
			/>
			<NumField id="laptime_warm_track_full_tank" label="Warm track (full tank)" default_value={default_warm_track_full_tank}
				onupdate={(value) => {
					if (!isNaN(value)) {
						console.log("Laptime warm track (full tank): ", value);
						set_laptime_warm_track_full_tank(value);
					}
				}}
			/>
			<NumField id="laptime_cold_track_old_tyres" label="Cold track (old tyres)" default_value={default_cold_track_old_tyres}
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