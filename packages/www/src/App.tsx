import React from 'react';

import AppImpl from './components/app-impl';

import { StarknetProvider } from '@starknet-react/core';

const App = () => (
	<StarknetProvider>
		<AppImpl />
	</StarknetProvider>
);

export default App;
