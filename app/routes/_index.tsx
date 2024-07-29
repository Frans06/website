import {
  MeshPortalMaterial,
  Plane,
  ScrollControls,
  Stars,
  Text,
  useCursor,
  useScroll,
} from "@react-three/drei";
import { ForwardRefComponent } from "@react-three/drei/helpers/ts-utils";
import {
  Canvas,
  GroupProps,
  useFrame,
  useThree,
  context as fiberContext,
} from "@react-three/fiber";
import type { MetaFunction } from "@remix-run/node";
import { useNavigate, useSearchParams } from "@remix-run/react";
import { easing } from "maath";
import {
  ReactNode,
  forwardRef,
  useContext,
  useImperativeHandle,
  useMemo,
  useRef,
  useState,
} from "react";
import * as ReactDOM from "react-dom/client";
import { useTranslation } from "react-i18next";
import { FiMenu } from "react-icons/fi";
import { DoubleSide, Vector3 } from "three";

export const meta: MetaFunction = () => [{ title: "Remix Notes" }];

function Frame({
  identifier,
  name,
  children,
  ...props
}: {
  identifier: string;
  bg: string;
  width?: number;
  height?: number;
  children: ReactNode;
} & GroupProps) {
  const portal = useRef(null!);
  const [params, setParams] = useSearchParams();
  const [hovered, hover] = useState(false);
  useCursor(hovered);

  useFrame((_, dt) =>
    easing.damp(
      portal.current,
      "blend",
      params.get("id") === identifier ? 1 : 0,
      0.2,
      dt,
    ),
  );

  return (
    <group {...props}>
      <Text
        font={"fonts/inter.ttf"}
        fontSize={0.15}
        anchorY="top"
        anchorX="left"
        lineHeight={0.8}
        fontWeight={500}
        position={[-0.5, 0.7, 0]}
        material-toneMapped={false}
      >
        {name}
      </Text>
      <mesh
        name={identifier}
        onClick={(e) => {
          e.stopPropagation();
          setParams((prev) => {
            prev.set("id", identifier);
            return prev;
          });
        }}
        onPointerOver={() => hover(true)}
        onPointerOut={() => hover(false)}
      >
        <planeGeometry />
        <MeshPortalMaterial ref={portal} side={DoubleSide}>
          <ambientLight />
          {children}
        </MeshPortalMaterial>
      </mesh>
    </group>
  );
}
function Rig() {
  const scroll = useScroll();
  const { camera, scene } = useThree();
  const [params] = useSearchParams();
  useFrame((_, dt) => {
    const active = scene.getObjectByName(params.get("id") || "undefined");
    if (!active) {
      easing.dampE(
        camera.rotation,
        [0, Math.PI * 2 * scroll.offset, 0],
        0.5,
        dt,
      );
      easing.damp3(camera.position, new Vector3(0, 0, 5), 0.5, dt);
    } else {
      const position = new Vector3(0, 0, 0.25);
      const focus = new Vector3(0, 0, 0);
      active.parent?.localToWorld(position);
      active.parent?.localToWorld(focus);
      easing.dampE(
        camera.rotation,
        [0, (Math.PI / 2) * Number(params.get("id") || 0), 0],
        0.5,
        dt,
      );
      easing.damp3(camera.position, position, 0.5, dt);
    }
  });
  return null;
}

// function Cam() {
//   const ref = useRef<PerspectiveCameraImpl>(null!);
//   const state = useScroll();
//
//   const { width } = useThree((state) => state.viewport);
//   useFrame(() => {
//     ref.current.position.x = width * (state.pages - 1) * state.offset; // Move camera
//     ref.current.updateProjectionMatrix();
//   });
//
//   return (
//     <PerspectiveCamera
//       makeDefault
//       ref={ref}
//       position={[0, 0, 5]}
//       fov={75}
//     ></PerspectiveCamera>
//   );
// }

export const CustomScrollHtml: ForwardRefComponent<
  { children: ({ scrollPos }: { scrollPos: number }) => React.ReactNode },
  HTMLDivElement
> = forwardRef(function InnerScroll(
  {
    children,
    ...props
  }: { children: ({ scrollPos }: { scrollPos: number }) => React.ReactNode },
  ref,
) {
  const state = useScroll();
  const group = useRef<HTMLDivElement>(null!);
  useImperativeHandle(ref, () => group.current, []);
  const fiberState = useContext(fiberContext);
  const [scrollPos, setScrollPos] = useState<number>(0);
  const root = useMemo(() => ReactDOM.createRoot(state.fixed), [state.fixed]);
  useFrame(() => {
    setScrollPos(state.offset);
  });
  const CustomComponent = () => (
    <div
      ref={group}
      style={{
        position: "absolute",
        top: 0,
        left: 0,
        willChange: "transform",
      }}
      {...props}
    >
      <fiberContext.Provider value={fiberState}>
        {children({ scrollPos })}
      </fiberContext.Provider>
    </div>
  );
  root.render(<CustomComponent />);
  return null;
});

export default function Index() {
  const { t } = useTranslation();
  const links = [
    { id: "0", content: t("home.projects.title") },
    { id: "1", content: t("home.blog.title") },
    { id: "2", content: t("home.me.title") },
    { id: "3", content: t("home.fun.title") },
  ];
  const navigate = useNavigate();
  return (
    <main className="w-full h-svh bg-black">
      <Canvas dpr={[1, 2]}>
        <ambientLight />
        <ScrollControls
          prepend
          horizontal
          pages={4}
          infinite
          damping={0.25}
          eps={0.0000001}
        >
          <Frame
            name={t("home.projects.title")}
            identifier={"0"}
            bg={"red"}
            position={[0, 0, 0]}
            scale={3}
          >
            <Plane position={[0, 0, -2]}>
              <meshStandardMaterial color="red" />
            </Plane>
            <Text position={[0, 0, -1.5]}>1</Text>
          </Frame>
          <Frame
            identifier={"1"}
            rotation={[0, Math.PI / 2, 0]}
            bg={"red"}
            position={[-5, 0, 5]}
            scale={3}
            name={"Blog"}
          >
            <Plane position={[0, 0, -2]}>
              <meshStandardMaterial color="hotpink" />
            </Plane>
            <Text position={[0, 0, -1.5]}>2</Text>
          </Frame>
          <Frame
            identifier={"2"}
            bg={"red"}
            rotation={[0, Math.PI, 0]}
            position={[0, 0, 10]}
            scale={3}
            name={"Resume"}
          >
            <Plane position={[0, 0, -2]}>
              <meshStandardMaterial color="green" />
            </Plane>
            <Text position={[0, 0, -1.5]}>3</Text>
          </Frame>
          <Frame
            identifier={"3"}
            bg={"red"}
            rotation={[0, -Math.PI / 2, 0]}
            position={[5, 0, 5]}
            scale={3}
            name={"Fun"}
          >
            <Plane position={[0, 0, -2]}>
              <meshStandardMaterial color="blue" />
            </Plane>
            <Text position={[0, 0, -1.5]}>4</Text>
          </Frame>
          <Rig />
          <CustomScrollHtml>
            {({ scrollPos }) => (
              <div className={"absolute top-0 flex h-svh w-svw"}>
                <div className="lg:flex text-white absolute  bottom-0 left-1/2 -translate-x-1/2 max-w-7xl border-t-2 border-x-2 bg-gray-600/60  border-white h-16 w-full overflow-hidden rounded-t-full hidden">
                  {links.map((link, key) => {
                    return (
                      <button
                        className={`lg:text-2xl text-lg flex flex-1 justify-center h-full items-center ${Math.floor(scrollPos * 4) === key ? "bg-gray-600" : "bg-transparent"}`}
                        onClick={() => navigate({ search: `?id=${link.id}` })}
                        key={link.id}
                      >
                        {link.content}
                      </button>
                    );
                  })}
                </div>
                <div
                  className={
                    "flex absolute sm:hidden text-white items-center justify-center rounded-full h-16 w-16 border-white border-2  bg-gray-600/60 right-4 top-4"
                  }
                >
                  <FiMenu className="h-8 w-8" />
                </div>
              </div>
            )}
          </CustomScrollHtml>
        </ScrollControls>
        <Stars />
      </Canvas>
    </main>
  );
}
