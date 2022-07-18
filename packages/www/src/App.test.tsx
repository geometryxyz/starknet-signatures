import React from 'react';
import { render, act } from '@testing-library/react';
import App from './App';

jest.mock('store/web-worker-demo/worker-caller');

it('doesnt crash', async () => {
	await act(async () => {
		render(<App />);
	});
});
