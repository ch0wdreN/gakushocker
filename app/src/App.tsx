import type { Component } from 'solid-js';
import { lazy } from 'solid-js';
import { useRoutes, Router } from '@solidjs/router';

const menu = lazy(async () => await import('@p/Menu'));
const item = lazy(async () => await import('@p/[id]'));

const routes = [
  {
    path: '/',
    component: menu,
  },
  {
    path: '/menu/:id',
    component: item,
  },
];

const App: Component = () => {
  const Routes = useRoutes(routes);
  return (
    <>
      <header class="bg-white shadow">
        <div class="mx-auto max-w-7xl py-6 px-4 sm:px-6 lg:px-8">
          <a href="/">
            <h1 class="text-3xl font-bold tracking-tight text-gray-900">
              学ショッカー
            </h1>
          </a>
        </div>
      </header>
      <Router>
        <Routes />
      </Router>
    </>
  );
};

export default App;
