import { twMerge } from "tailwind-merge";

interface TitleProps {
  level?: 1 | 2 | 3 | 4;
  isCustom?: boolean;
  size?: "2xs" | "xs" | "sm" | "md" | "lg" | "xl" | "2xl";
  children: JSX.Element | string | null;
  className?: string;
}

export const Title = ({
  level = 1,
  size = "lg",
  children,
  isCustom = false,
  className: propsClassName,
}: TitleProps) => {
  let className = "font-inter";
  if (!isCustom) {
    switch (size) {
      case "2xs":
        className =
          "font-bold font-inter text-base text-nipon tracking-[-0.01em] sm:text-lg";
        break;
      case "xs":
        className =
          "font-bold font-inter text-lg text-nipon tracking-[-0.01em] sm:text-xl";
        break;

      case "sm":
        className =
          "font-bold font-inter text-2xl text-nipon tracking-[-0.01em] sm:text-3xl";
        break;
      case "md":
        className =
          "font-bold font-inter text-3xl text-nipon tracking-[-0.01em] sm:text-4xl";
        break;
      case "lg":
        className =
          "font-bold font-inter text-4xl text-nipon tracking-[-0.01em] sm:text-5xl";
        break;
      case "xl":
        className =
          "font-bold font-inter text-5xl text-nipon tracking-[-0.01em] sm:text-6xl";
        break;
      case "2xl":
        className =
          "font-bold font-inter text-6xl text-nipon tracking-[-0.01em] sm:text-7xl";
        break;
    }
  }
  const CustomTag = `h${0 < level && level < 7 ? level : 6}` as keyof Pick<
    JSX.IntrinsicElements,
    "h1" | "h2" | "h3" | "h4" | "h5" | "h6"
  >;

  return (
    <CustomTag className={twMerge(className, propsClassName)}>
      {children}
    </CustomTag>
  );
};
