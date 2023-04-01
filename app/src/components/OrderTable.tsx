import {
  Heading,
  Table,
  TableContainer,
  Tbody,
  Td,
  Tfoot,
  Th,
  Thead,
  Tr,
} from '@chakra-ui/react';
import { useRecoilValue } from 'recoil';
import { cartState, totalState } from '@/recoil/cart';

export const OrderTable = () => {
  let total = useRecoilValue(totalState);
  let cart = useRecoilValue(cartState);
  return (
    <>
      <TableContainer>
        <Table variant={'simple'}>
          <Thead>
            <Tr>
              <Th>商品名</Th>
              <Th>単価</Th>
              <Th>個数</Th>
            </Tr>
          </Thead>
          <Tbody>
            {cart.map((item, index) => {
              return (
                <Tr key={index}>
                  <Td>{item.name}</Td>
                  <Td>{item.price}</Td>
                  <Td>{item.quantity}</Td>
                </Tr>
              );
            })}
          </Tbody>
          <Tfoot>
            <Tr>
              <Th>合計 {total}円</Th>
            </Tr>
          </Tfoot>
        </Table>
      </TableContainer>
    </>
  );
};
