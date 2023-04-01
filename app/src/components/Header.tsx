import {
  Box,
  Avatar,
  Flex,
  Link,
  Menu,
  MenuButton,
  Button,
  Center,
  MenuList,
  MenuDivider,
  MenuItem,
  Spacer,
  ButtonGroup,
  Heading,
  Icon,
  useDisclosure,
  ModalOverlay,
  ModalBody,
  ModalHeader,
  ModalContent,
  ModalFooter,
  Modal,
  ModalCloseButton,
  Text,
} from '@chakra-ui/react';
import { useRouter } from 'next/router';
import { BiCart } from 'react-icons/bi';
import { useRecoilValue } from 'recoil';
import { cartState } from '@/recoil/cart';
import { isSigninState } from '@/recoil/signin';
import { userState } from '@/recoil/user';

export const Header = () => {
  let signin = useRecoilValue(isSigninState);
  let user = useRecoilValue(userState);
  let cart = useRecoilValue(cartState);
  let router = useRouter();
  const { isOpen, onOpen, onClose } = useDisclosure();
  return (
    <Box w="100%" px={4}>
      <Flex alignItems={'center'} justifyItems={'space-between'} h={16} gap={2}>
        <Box>
          <Heading>学ショッカー</Heading>
        </Box>
        <Spacer />
        {signin ? (
          <Flex gap={2}>
            <Icon as={BiCart} onClick={onOpen} w={8} h={8}>
              Open Modal
            </Icon>

            <Modal isOpen={isOpen} onClose={onClose}>
              <ModalOverlay />
              <ModalContent>
                <ModalHeader>Modal Title</ModalHeader>
                <ModalCloseButton />
                <ModalBody>
                  {cart.map((i, index) => {
                    return (
                      <Text key={index}>
                        {i.name}
                        {i.quantity}
                      </Text>
                    );
                  })}
                </ModalBody>

                <ModalFooter>
                  <Button colorScheme="blue" mr={3} onClick={onClose}>
                    閉じる
                  </Button>
                  <Button
                    variant="ghost"
                    onClick={() => {
                      onClose();
                      router.push('/confirm');
                    }}
                  >
                    購入へ進む
                  </Button>
                </ModalFooter>
              </ModalContent>
            </Modal>
            <Menu>
              <MenuButton
                as={Button}
                rounded={'full'}
                variant={'link'}
                cursor={'pointer'}
                minW={0}
              >
                <Avatar size={'sm'} src={'https://bit.ly/broken-link'} />
              </MenuButton>
              <MenuList alignItems={'center'}>
                <br />
                <Center>
                  <Avatar size={'2xl'} src={'https://bit.ly/broken-link'} />
                </Center>
                <br />
                <Center>
                  <p>{user.displayName}</p>
                </Center>
                <br />
                <MenuDivider />
                <MenuItem>Your Servers</MenuItem>
                <MenuItem>Account Settings</MenuItem>
                <MenuItem>Logout</MenuItem>
              </MenuList>
            </Menu>
          </Flex>
        ) : (
          <ButtonGroup gap={2}>
            <Button as={Link} href={'/auth/signin'} colorScheme={'teal'}>
              サインイン
            </Button>
            <Button as={Link} href={'/auth/signup'} colorScheme={'teal'}>
              サインアップ
            </Button>
          </ButtonGroup>
        )}
      </Flex>
    </Box>
  );
};
