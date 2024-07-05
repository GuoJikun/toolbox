import { Menu, MenuItem, Submenu } from "@tauri-apps/api/menu";
import { getCurrent } from "@tauri-apps/api/window";
import { getCurrent as getWebviewWindow } from "@tauri-apps/api/webviewWindow";

interface MenuChild {
  label: string;
  value?: string;
  action?: (value: string) => void;
}

interface SubMenu {
  label: string;
  children: Array<MenuChild>;
}

export default () => {
  const setMenu = async (
    menuList: Array<SubMenu | MenuChild>,
    type: "window" | "webview" | "app"
  ) => {
    

    const menuIt = menuList.map(async (menu: SubMenu | MenuChild) => {
        // @ts-ignore
      const { children = null, ...item } = menu;
      if (children) {
        const tmp = { ...item } as SubMenu;
        return await Submenu.new({
          text: tmp?.label,
          items: children.map((c: MenuChild) =>
            MenuItem.new({
              text: c.label,
              id: c.value,
              action: c.action,
            })
          ),
        });
      } else {
        const tmp = { ...item } as MenuChild;
        return MenuItem.new({
          text: tmp?.label,
          id: tmp?.value,
          action: tmp?.action,
        });
      }
    });

    const menus = await Menu.new({
        items: menuIt as Array<any>
    });
    if (type === "window") {
      menus.setAsWindowMenu(getCurrent());
    } else if (type === "webview") {
      menus.setAsWindowMenu(getWebviewWindow());
    } else if (type === "app") {
      menus.setAsAppMenu();
    }
  };
  return { setMenu };
};
