import React from 'react'
import { render, fireEvent, wait } from '@testing-library/react'
import { RegisterForm } from './register';

describe('The Register User View', () => {
  test('Rendering when not pending', () => {
    const { container } = render(<RegisterForm username="testuser" onSubmit={async () => { }} onCancel={() => { }} />);
    expect(container).toMatchSnapshot();
  });

  test('Entering details', () => {
    const { container, getByLabelText } = render(<RegisterForm username="testuser" onSubmit={async () => { }} onCancel={() => { }} />);

    fireEvent.change(getByLabelText('Email Address'), { target: { value: 'test@example.com' } });
    fireEvent.change(getByLabelText('Display Name'), { target: { value: 'Test User' } });
    fireEvent.change(getByLabelText('Password'), { target: { value: 'Pa55word' } });
    fireEvent.change(getByLabelText('Repeat Password'), { target: { value: 'Pa55word' } });
    expect(container).toMatchSnapshot();
  });

  describe('Submitting the form', () => {

    test('Without filling anything out', async () => {
      const onSubmit = jest.fn()
      const { container, getByText } = render(<RegisterForm username="testuser" onSubmit={onSubmit} onCancel={() => { }} />);
      fireEvent.click(getByText('Register', { selector: 'button' }));

      await wait(() => expect(onSubmit).toHaveBeenCalledTimes(0));
      expect(container).toMatchSnapshot();
    });

    test('With whitespace only email address', async () => {
      const onSubmit = jest.fn()
      const { container, getByLabelText, getByText } = render(<RegisterForm username="testuser" onSubmit={onSubmit} onCancel={() => { }} />);
      fireEvent.change(getByLabelText('Email Address'), { target: { value: '   ' } });
      fireEvent.change(getByLabelText('Display Name'), { target: { value: 'Test User' } });
      fireEvent.change(getByLabelText('Password'), { target: { value: 'Pa55word' } });
      fireEvent.change(getByLabelText('Repeat Password'), { target: { value: 'Pa55word' } });
      fireEvent.click(getByText('Register', { selector: 'button' }));

      await wait(() => expect(onSubmit).toHaveBeenCalledTimes(0));
      expect(container).toMatchSnapshot();
    });

    test('With an invalid email address', async () => {
      const onSubmit = jest.fn()
      const { container, getByLabelText, getByText } = render(<RegisterForm username="testuser" onSubmit={onSubmit} onCancel={() => { }} />);
      fireEvent.change(getByLabelText('Email Address'), { target: { value: 'testuser' } });
      fireEvent.change(getByLabelText('Display Name'), { target: { value: 'Test User' } });
      fireEvent.change(getByLabelText('Password'), { target: { value: 'Pa55word' } });
      fireEvent.change(getByLabelText('Repeat Password'), { target: { value: 'Pa55word' } });
      fireEvent.click(getByText('Register', { selector: 'button' }));

      await wait(() => expect(onSubmit).toHaveBeenCalledTimes(0));
      expect(container).toMatchSnapshot();
    });

    test('With whitespace only display name', async () => {
      const onSubmit = jest.fn()
      const { container, getByLabelText, getByText } = render(<RegisterForm username="testuser" onSubmit={onSubmit} onCancel={() => { }} />);
      fireEvent.change(getByLabelText('Email Address'), { target: { value: 'test@example.com' } });
      fireEvent.change(getByLabelText('Display Name'), { target: { value: ' ' } });
      fireEvent.change(getByLabelText('Password'), { target: { value: 'Pa55word' } });
      fireEvent.change(getByLabelText('Repeat Password'), { target: { value: 'Pa55word' } });
      fireEvent.click(getByText('Register', { selector: 'button' }));

      await wait(() => expect(onSubmit).toHaveBeenCalledTimes(0));
      expect(container).toMatchSnapshot();
    });

    test('With mismatched passwords', async () => {
      const onSubmit = jest.fn()
      const { container, getByLabelText, getByText } = render(<RegisterForm username="testuser" onSubmit={onSubmit} onCancel={() => { }} />);
      fireEvent.change(getByLabelText('Email Address'), { target: { value: 'test@example.com' } });
      fireEvent.change(getByLabelText('Display Name'), { target: { value: 'Test User' } });
      fireEvent.change(getByLabelText('Password'), { target: { value: 'Password' } });
      fireEvent.change(getByLabelText('Repeat Password'), { target: { value: 'Pa55word' } });
      fireEvent.click(getByText('Register', { selector: 'button' }));

      await wait(() => expect(onSubmit).toHaveBeenCalledTimes(0));
      expect(container).toMatchSnapshot();
    });

    test('Successfully', async () => {
      const onSubmit = jest.fn()
      const { container, getByLabelText, getByText } = render(<RegisterForm username="testuser" onSubmit={onSubmit} onCancel={() => { }} />);
      fireEvent.change(getByLabelText('Email Address'), { target: { value: 'test@example.com' } });
      fireEvent.change(getByLabelText('Display Name'), { target: { value: 'Test User' } });
      fireEvent.change(getByLabelText('Password'), { target: { value: 'Pa55word' } });
      fireEvent.change(getByLabelText('Repeat Password'), { target: { value: 'Pa55word' } });
      fireEvent.click(getByText('Register', { selector: 'button' }));

      await wait(() => expect(onSubmit).toHaveBeenCalledTimes(1));
      expect(onSubmit).toHaveBeenCalledWith('testuser', 'test@example.com', 'Test User', 'Pa55word');
      expect(container).toMatchSnapshot();
    });

    test('With padded values', async () => {
      const onSubmit = jest.fn()
      const { container, getByLabelText, getByText } = render(<RegisterForm username="testuser" onSubmit={onSubmit} onCancel={() => { }} />);
      fireEvent.change(getByLabelText('Email Address'), { target: { value: '  test@example.com  ' } });
      fireEvent.change(getByLabelText('Display Name'), { target: { value: '  Test User  ' } });
      fireEvent.change(getByLabelText('Password'), { target: { value: '  Pa55word  ' } });
      fireEvent.change(getByLabelText('Repeat Password'), { target: { value: '  Pa55word  ' } });
      fireEvent.click(getByText('Register', { selector: 'button' }));

      await wait(() => expect(onSubmit).toHaveBeenCalledTimes(1));
      expect(onSubmit).toHaveBeenCalledWith('testuser', 'test@example.com', 'Test User', '  Pa55word  ');
      expect(container).toMatchSnapshot();
    });
  });
});
