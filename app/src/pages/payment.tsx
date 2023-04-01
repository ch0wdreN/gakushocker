import { Button, Heading, Radio, RadioGroup, Stack } from '@chakra-ui/react';
import { useRouter } from 'next/router';
import { useRecoilValue } from 'recoil';
import { userState } from '@/recoil/user';

const Payment = () => {
  let user = useRecoilValue(userState);
  let router = useRouter();
  return (
    <>
      <Heading textAlign={'center'}>お支払い情報</Heading>
      <RadioGroup>
        <Stack>
          <Radio>保有ポイントで支払う 残高 {user.point} pt</Radio>
          <Radio>PayPayで支払う</Radio>
        </Stack>
      </RadioGroup>
      <Button onClick={() => router.push('/')}>支払いを完了する</Button>
    </>
  );
};

export default Payment;
