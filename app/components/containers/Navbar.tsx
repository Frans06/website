import { useNavigate } from "@remix-run/react";
import { useTranslation } from "react-i18next";

export const Navbar = () => {
  const navigate = useNavigate();
  const { t } = useTranslation();
  const links = [
    { id: "0", content: t("home.projects.title") },
    { id: "1", content: t("home.blog.title") },
    { id: "2", content: t("home.fun.title") },
    { id: "3", content: t("home.me.title") },
  ];
  return (
    <div className="lg:flex text-white absolute  bottom-0 left-1/2 -translate-x-1/2 max-w-7xl border-t-2 border-x-2 bg-gray-600/60  border-white h-16 w-full overflow-hidden rounded-t-full hidden">
      {links.map((link) => {
        return (
          <button
            className="lg:text-2xl text-lg flex flex-1 justify-center h-full items-center"
            onClick={() => navigate({ search: `?id=${link.id}` })}
            key={link.id}
          >
            {link.content}
          </button>
        );
      })}
    </div>
  );
};
