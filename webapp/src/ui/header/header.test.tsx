import React from 'react'
import { render } from '@testing-library/react'
import { BrowserRouter as Router } from 'react-router-dom';
import { Header } from './header';
import '../../i18n';

test('Rendering the header bar', () => {
  const { container } = render(<Router><Header /></Router>);
  expect(container).toMatchSnapshot();
});
