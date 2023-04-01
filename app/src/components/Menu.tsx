import { Link, SimpleGrid, Container, Heading } from '@chakra-ui/react';
import { useListProductQuery, Product } from '../../generated/graphql';
import { MenuItem } from '@/components/MenuItem';

export const Menu = () => {
  const [result] = useListProductQuery();
  if (!result.data) {
    return <Link href="/auth/signin">サインイン</Link>;
  }
  return (
    <>
      <Heading textAlign={'center'}>メニュー</Heading>
      <Container maxW="8xl" marginTop={'4'}>
        <SimpleGrid minChildWidth={'416px'} spacing={6}>
          {result.data.listProduct.map((product: Product, key) => {
            return <MenuItem key={key} {...product} />;
          })}
        </SimpleGrid>
      </Container>
    </>
  );
};
