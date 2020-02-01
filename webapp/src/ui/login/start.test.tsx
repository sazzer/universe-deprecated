import React from 'react'
import { render, fireEvent, wait } from '@testing-library/react'
import { StartLoginForm } from './start';

describe('The Start Login View', () => {
  test('Rendering when not pending', () => {
    const { container } = render(<StartLoginForm onSubmit={async () => { }} />);
    expect(container).toMatchSnapshot();
  });

  test('Entering a username', () => {
    const { container, getByLabelText } = render(<StartLoginForm onSubmit={async () => { }} />);

    fireEvent.change(getByLabelText('Username'), { target: { value: 'testuser' } });
    expect(container).toMatchSnapshot();
  });

  test('Entering a whitespace-only username', () => {
    const { container, getByLabelText } = render(<StartLoginForm onSubmit={async () => { }} />);

    fireEvent.change(getByLabelText('Username'), { target: { value: '  ' } });
    expect(container).toMatchSnapshot();
  });

  describe('Submitting a username', () => {

    test('Successfully', async () => {
      const onSubmit = jest.fn()
      const { container, getByLabelText, getByText } = render(<StartLoginForm onSubmit={onSubmit} />);

      fireEvent.change(getByLabelText('Username'), { target: { value: 'testuser' } });
      fireEvent.click(getByText('Login / Register', { selector: 'button' }));

      await wait(() => expect(onSubmit).toHaveBeenCalledTimes(1));
      expect(onSubmit).toHaveBeenCalledWith('testuser');
      expect(container).toMatchSnapshot();
    });

    test('With a network error', async () => {
      const onSubmit = jest.fn(async () => {
        throw new Error('Network Error');
      });
      const { container, getByLabelText, getByText } = render(<StartLoginForm onSubmit={onSubmit} />);

      fireEvent.change(getByLabelText('Username'), { target: { value: 'testuser' } });
      fireEvent.click(getByText('Login / Register', { selector: 'button' }));

      await wait(() => expect(onSubmit).toHaveBeenCalledTimes(1));
      expect(onSubmit).toHaveBeenCalledWith('testuser');
      expect(container).toMatchSnapshot();
    });

    test('Submitting a blank username', async () => {
      const onSubmit = jest.fn()
      const { container, getByText } = render(<StartLoginForm onSubmit={onSubmit} />);

      fireEvent.click(getByText('Login / Register', { selector: 'button' }));

      await wait(() => expect(onSubmit).toHaveBeenCalledTimes(0));
      expect(container).toMatchSnapshot();
    });

    test('Submitting a whitespace-only username', async () => {
      const onSubmit = jest.fn()
      const { container, getByLabelText, getByText } = render(<StartLoginForm onSubmit={onSubmit} />);

      fireEvent.change(getByLabelText('Username'), { target: { value: '  ' } });
      fireEvent.click(getByText('Login / Register', { selector: 'button' }));

      await wait(() => expect(onSubmit).toHaveBeenCalledTimes(0));
      expect(container).toMatchSnapshot();
    });
  });

});
