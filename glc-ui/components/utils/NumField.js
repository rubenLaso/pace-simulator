import * as React from 'react';
import Box from '@mui/material/Box';
import TextField from '@mui/material/TextField';

export const NumField = ({ field_id, label, default_value, onupdate }) => {
	return (
		<Box
			// component="form"
			sx={{
				'& > :not(style)': { m: 1, width: '25ch' },
			}}
			noValidate
			autoComplete="off"
			onChange={(event) => {
				const value = parseFloat(event.target.value);
				if (typeof onupdate === 'function') {
					onupdate(value);
				}
			}}
		>
			<TextField id={field_id} label={label} type="number" variant="filled" defaultValue={default_value} />
		</Box >
	);
}