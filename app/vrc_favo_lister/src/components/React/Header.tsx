// import {
//   Heading,
//   Text,
//   IconButton,
//   Drawer,
//   DrawerCloseButton,
//   DrawerHeader,
//   DrawerBody,
//   DrawerFooter,
//   useDisclosure,
//   HStack,
//   VStack,
//   Spacer,
//   Divider,
// } from "@yamada-ui/react"
// import { Icon as FontAwesomeIcon } from "@yamada-ui/fontawesome"
// import { faBars } from "@fortawesome/free-solid-svg-icons"

// export default function Header() {
//   /* ドロワー */
//   const { isOpen, onOpen, onClose } = useDisclosure()

//   return (
//     <HStack as="header" bg="secondary">
//       <Heading ml="sm" size="xl">
//         <IconButton
//           icon={<FontAwesomeIcon icon={faBars} />}
//           variant="outline"
//           size="lg"
//           onClick={() => {
//             onOpen()
//           }}
//         />
//       </Heading>

//       { /* ページ上部のナビゲーションバー
//         <Link href="/">{config.siteName}</Link>
//         */
//       }
//       {
//         /*
//         config.nav.map((item, index) => (
//           <Link
//             key={index}
//             href={item.path}
//             >【{item.name}】</Link>
//         ))
//         */
//       }

//       <Spacer />

//       { /* ドロワー本体 */}
//       <Drawer isOpen={isOpen} onClose={onClose} size="md" placement="left">
//         <DrawerCloseButton color="red.500" />
//         <DrawerHeader>
//           <VStack>
//             <Text>Reserved Tasks</Text>
//             <Divider />
//           </VStack>
//         </DrawerHeader>
//         <DrawerBody>

//         </DrawerBody>
//         <DrawerFooter>
//           <VStack>
//             <Divider />
//             {/*
//             <Link
//               href="https://ofuse.me/"
//               target="_blank">
//               <span>作者に応援を送る</span>
//             </Link>
//             */}
//           </VStack>
//         </DrawerFooter>
//       </Drawer>
//     </HStack>
//   )
// }
