import Head from 'next/head';
import { Menu } from '@/components/Menu';

const Home = () => {
  return (
    <>
      <Head>
        <title>学ショッカー</title>
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main>
        <Menu />
      </main>
    </>
  );
};

export default Home;
