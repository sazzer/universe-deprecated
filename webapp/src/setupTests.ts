import '@testing-library/jest-dom/extend-expect';
import { configure } from '@testing-library/react'
import './i18n';
import 'mutationobserver-shim';

configure({ testIdAttribute: 'data-test' })
