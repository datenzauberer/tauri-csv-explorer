import { getName, getVersion } from "@tauri-apps/api/app";
// for handling custom menu items
import { emit } from "@tauri-apps/api/event";
import {
    type AboutMetadata,
    Menu,
    MenuItem,
    PredefinedMenuItem,
    Submenu,
} from "@tauri-apps/api/menu";
import { platform } from "@tauri-apps/plugin-os";

const currentPlatform = platform();

/**
 * Platform Specific custom menu,
 * File > Open
 */
export async function setCustomMenu() {
    const menuItems = [
        await createSubmenuFile(),
        await createSubmenuView(),
        await createSubmenuWindow(),
        await createSubmenuHelp(),
    ];

    // Add the "About" submenu only on macOS
    if (currentPlatform === "macos") {
        menuItems.unshift(await createSubmenuAbout());
    }

    // Build and Set Menu
    const menu = await Menu.new({
        items: menuItems,
    });
    await menu.setAsAppMenu();
}

const accelerator = "CmdOrCtrl+O";

export function getDisplayShortcutFileOpen(): string {
    return getCmdOrCtrlString(accelerator);
}

// REQ-005 Menue Open File
async function createSubmenuFile() {
    const openItem = await MenuItem.new({
        id: "open_item",
        text: "Open",
        accelerator: accelerator,
        action: () => {
            emit("menu_action_open_file", {});
        },
    });

    const fileMenuItems = [
        openItem,
        await PredefinedMenuItem.new({
            item: "CloseWindow",
        }),
    ];

    // Add the "Quit" menu item only if the platform is not macOS
    if (currentPlatform !== "macos") {
        fileMenuItems.push(
            await PredefinedMenuItem.new({
                text: `Quit ${await getName()}`,
                item: "Quit",
            })
        );
    }

    return await Submenu.new({
        text: "File",
        items: fileMenuItems,
    });
}

async function createSubmenuView() {
    const menuItems = [];
    if (currentPlatform === "macos") {
        menuItems.push(await PredefinedMenuItem.new({ item: "Fullscreen" }));
    }

    return await Submenu.new({
        text: "View",
        items: menuItems,
    });
}

async function createSubmenuWindow() {
    const menuItems = [
        await PredefinedMenuItem.new({ item: "Minimize" }),
        await PredefinedMenuItem.new({ item: "Maximize" }),
        await PredefinedMenuItem.new({ item: "Separator" }),
    ];
    if (currentPlatform === "macos") {
        menuItems.push(await PredefinedMenuItem.new({ item: "Separator" }));
    }
    menuItems.push(await PredefinedMenuItem.new({ item: "CloseWindow" }));

    return await Submenu.new({
        text: "Window",
        items: menuItems,
    });
}

async function createSubmenuHelp() {
    const menuItems = [];
    if (currentPlatform !== "macos") {
        menuItems.push(await createAboutItem());
    }

    return await Submenu.new({
        text: "Help",
        items: menuItems,
    });
}

async function createSubmenuAbout() {
    // Will become the application submenu on MacOS
    return await Submenu.new({
        text: "About",
        items: [
            await createAboutItem(),
            await PredefinedMenuItem.new({
                item: "Separator",
            }),
            await PredefinedMenuItem.new({
                item: "Services",
            }),
            await PredefinedMenuItem.new({
                item: "Separator",
            }),
            await PredefinedMenuItem.new({
                text: `Hide ${await getName()}`,
                item: "Hide",
            }),
            await PredefinedMenuItem.new({
                item: "HideOthers",
            }),
            await PredefinedMenuItem.new({
                item: "Separator",
            }),
            await PredefinedMenuItem.new({
                text: `Quit ${await getName()}`,
                item: "Quit",
            }),
        ],
    });
}

async function createAboutItem() {
    // You don’t have direct `app_handle` in TS, but you can pull package info
    // from the environment variables injected by Tauri.
    const name = await getName();
    const version = await getVersion();
    // SMTODO: copyright info cannot be accessed from TypeScript (workaround over invole)
    const copyright = "MIT License"; // SMWORKAROUND copyright
    const publisher = "SMINFO authors";

    const aboutMetadata: AboutMetadata = {
        name,
        version,
        copyright: copyright ?? undefined,
        authors: publisher ? [publisher] : undefined,
    };

    return await PredefinedMenuItem.new({
        text: `About ${name}`,
        item: {
            About: aboutMetadata,
        },
    });
}

/**
 * Replaces 'CmdOrCtrl' in an accelerator string with the platform-specific symbol.
 * @param accelerator The accelerator string (e.g., 'CmdOrCtrl+O').
 * @returns {Promise<string>} A promise that resolves to the formatted string (e.g., '⌘+O' or 'Ctrl+O').
 */
export function getCmdOrCtrlString(accelerator: string): string {
    return accelerator.replace("CmdOrCtrl", getCmdOrCtrlSymbol());
}

/**
 * Returns the platform-specific symbol for CmdOrCtrl.
 * @returns {string} '⌘' for macOS, 'Ctrl' for other platforms.
 */
export function getCmdOrCtrlSymbol(): string {
    // Check if the platform string includes "Mac"
    if (currentPlatform === "macos") {
        return "⌘";
    }
    // Otherwise, assume it's a Windows, Linux, or other Ctrl-based platform
    return "Ctrl";
}
