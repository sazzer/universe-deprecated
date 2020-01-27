import React from 'react'
import { render, fireEvent, wait } from '@testing-library/react'
import { StartLoginFormView, StartLoginForm } from './start';

describe('The Start Login View', () => {
  test('Rendering when not pending', () => {
    const { container } = render(<StartLoginFormView pending={false} onSubmit={() => { }} />);
    expect(container).toMatchSnapshot();
  });

  test('Rendering when pending', () => {
    const { container } = render(<StartLoginFormView pending={true} onSubmit={() => { }} />);
    expect(container).toMatchSnapshot();
  });

  test('Entering a username', () => {
    const { container, getByLabelText } = render(<StartLoginFormView pending={false} onSubmit={() => { }} />);

    fireEvent.change(getByLabelText('Username'), { target: { value: 'testuser' } });
    expect(container).toMatchSnapshot();
  });
});

describe('The Start Login Wrapper', () => {
  test('Initial Rendering', () => {
    const { container } = render(<StartLoginForm onSubmit={() => { }} />);
    expect(container).toMatchSnapshot();
  });

  test('Entering a username', () => {
    const { container, getByLabelText } = render(<StartLoginForm onSubmit={() => { }} />);

    fireEvent.change(getByLabelText('Username'), { target: { value: 'testuser' } });
    expect(container).toMatchSnapshot();
  });

  test('Submitting a username', async () => {
    const onSubmit = jest.fn()
    const { getByLabelText, getByText } = render(<StartLoginForm onSubmit={onSubmit} />);

    fireEvent.change(getByLabelText('Username'), { target: { value: 'testuser' } });
    fireEvent.click(getByText('Login / Register', { selector: 'button' }));

    await wait(() => expect(onSubmit).toHaveBeenCalledTimes(1));
    expect(onSubmit).toHaveBeenCalledWith('testuser', true);
  });
});
