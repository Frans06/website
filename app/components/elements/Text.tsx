import { twMerge } from "tailwind-merge";

interface TitleProps {
  isCustom?: boolean;
  size?: "sm" | "md" | "lg" | "xl";
  children: JSX.Element | string | null;
  className?: string;
}

export const Text = ({
  size = "lg",
  children,
  isCustom = false,
  className: propsClassName,
}: TitleProps) => {
  let className = "font-inter";
  if (!isCustom) {
    switch (size) {
      case "sm":
        className = "font-normal font-inter text-xs text-nipon sm:text-sm";
        break;
      case "md":
        className = "font-normal font-inter text-sm text-nipon  sm:text-base";
        break;
      case "lg":
        className = "font-normal font-inter text-base text-nipon  sm:text-lg";
        break;
      case "xl":
        className = "font-normal font-inter text-lg text-nipon  sm:text-xl";
        break;
    }
  }

  return typeof children === "string" ? (
    <p
      className={
        propsClassName ? twMerge(className, propsClassName) : className
      }
      dangerouslySetInnerHTML={{
        __html: children || "",
      }}
    />
  ) : (
    <p
      className={
        propsClassName ? twMerge(className, propsClassName) : className
      }
    >
      {children}
    </p>
  );
};
