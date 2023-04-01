import { Button, Heading } from '@chakra-ui/react';
import { useRouter } from 'next/router';
import { OrderTable } from '@/components/OrderTable';

const Confirm = () => {
  let router = useRouter();
  return (
    <>
      <Heading textAlign={'center'}>決済</Heading>
      <OrderTable />
      <Button variant={'ghost'} onClick={() => router.push('/payment')}>
        注文を確定する
      </Button>
    </>
  );
};

export default Confirm;
