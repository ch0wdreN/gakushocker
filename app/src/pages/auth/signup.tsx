import {
  Flex,
  Box,
  FormControl,
  FormLabel,
  Input,
  Stack,
  Link,
  Button,
  Heading,
  useColorModeValue,
} from '@chakra-ui/react';
import axios, { AxiosResponse } from 'axios';
import { useRouter } from 'next/router';
import { useState } from 'react';
import { useSetRecoilState } from 'recoil';
import { User } from '../../../generated/graphql';
import { isSigninState } from '@/recoil/signin';
import { userState } from '@/recoil/user';

const Signup = () => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [displayName, setDisplayName] = useState('');
  const setSignin = useSetRecoilState(isSigninState);
  const setUser = useSetRecoilState(userState);
  const API_URL = process.env.NEXT_PUBLIC_API_URL;
  const router = useRouter();
  let client = axios.create({ withCredentials: true });
  const auth = () => {
    client
      .post(`${API_URL}/auth/signup`, {
        email: email,
        password: password,
        display_name: displayName,
        point: 9999,
      })
      .then((res: AxiosResponse<User>) => {
        setSignin(true);
        setUser(res.data);
        router.push('/');
      });
  };
  return (
    <Flex minH={'100vh'} align={'center'} justify={'center'} bg={'white:800'}>
      <Stack
        spacing={8}
        mx={'auto'}
        maxW={'lg'}
        py={12}
        px={6}
        align={'center'}
      >
        <Heading fontSize={'4xl'}>サインアップ</Heading>
        <Box
          rounded={'lg'}
          bg={useColorModeValue('white', 'gray.700')}
          boxShadow={'lg'}
          p={8}
        >
          <Stack spacing={4}>
            <FormControl id="email">
              <FormLabel>Email address</FormLabel>
              <Input type="email" onChange={(e) => setEmail(e.target.value)} />
            </FormControl>

            <FormControl>
              <FormLabel>Display Name</FormLabel>
              <Input
                type="text"
                onChange={(e) => setDisplayName(e.target.value)}
              />
            </FormControl>
            <FormControl id="password">
              <FormLabel>Password</FormLabel>
              <Input
                type="password"
                onChange={(e) => setPassword(e.target.value)}
              />
            </FormControl>
            <Stack spacing={10}>
              <Stack
                direction={{ base: 'column', sm: 'row' }}
                align={'start'}
                justify={'space-between'}
              >
                <Link color={'blue.400'} href={'/auth/signin'}>
                  サインイン
                </Link>
              </Stack>
              <Button
                bg={'blue.400'}
                color={'white'}
                _hover={{
                  bg: 'blue.500',
                }}
                onClick={() => auth()}
              >
                Sign up
              </Button>
            </Stack>
          </Stack>
        </Box>
      </Stack>
    </Flex>
  );
};

export default Signup;
