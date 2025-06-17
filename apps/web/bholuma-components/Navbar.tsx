"use client";

import Image from "next/image";
import Link from "next/link";
import { FaGithub, FaSquareXTwitter, FaLinkedin } from "react-icons/fa6";
import { useEffect, useState } from "react";
import { Drawer, DrawerClose, DrawerContent, DrawerDescription, DrawerFooter, DrawerHeader, DrawerTitle, DrawerTrigger } from "@/components/ui/drawer";
import { Button } from "@/components/ui/button";
import { MenuIcon } from "lucide-react";
export const Navbar = () => {
    const [hasMounted, setHasMounted] = useState(false);
    useEffect(() => {
        setHasMounted(true);
    }, []);

    if (!hasMounted) return null; // ⛔️ Prevent hydration mismatch

    return (
        <nav className="top-0 z-50 w-full bg-gradient-to-b from-black/50 to-transparent backdrop-blur-md">
            <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between hidden md:flex">
                {/* Logo */}
                <Link href="/" className="flex items-center space-x-2">
                    <Image src="/bholumaIcon.png" alt="Bholuma" width={32} height={32} />
                    <span className="text-xl font-bold text-white">Bholuma DEX</span>
                </Link>

                {/* Social Icons */}
                <div className="flex space-x-6 items-center text-white">
                    <Link
                        href="https://x.com/sattyshivhare"
                        target="_blank"
                        className="hover:text-orange-500"
                    >
                        <FaSquareXTwitter size={20} />
                    </Link>
                    <Link
                        href="https://github.com/learnwithsatyam"
                        target="_blank"
                        className="hover:text-orange-500"
                    >
                        <FaGithub size={20} />
                    </Link>
                    <Link
                        href="https://linkedin.com/in/satyam-shivhare"
                        target="_blank"
                        className="hover:text-orange-500"
                    >
                        <FaLinkedin size={20} />
                    </Link>
                </div>
            </div>
            <div className="md:hidden flex items-center mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between ">
                <div className="w-1/2 flex items-center border-red-3 justify-start">
                    {/* Logo */}
                    <Link href="/" className="flex items-center space-x-2">
                        <Image src="/bholumaIcon.png" alt="Bholuma" width={32} height={32} />
                        <span className="text-xl font-bold text-white">Bholuma DEX</span>
                    </Link>
                </div>
                <Drawer>
                    <DrawerTrigger className="w-1/2 flex items-center justify-end text-black dark:text-white"><MenuIcon className="text-white" /></DrawerTrigger>
                    <DrawerContent className="dark">
                        <DrawerHeader>
                            <DrawerTitle className="text-lg font-semibold">Menu</DrawerTitle>
                        </DrawerHeader>
                        <div className="flex flex-col gap-4 px-4 py-2">
                            <Link href="https://x.com/sattyshivhare" target="_blank" className="text-sm text-muted-foreground hover:text-orange-500 flex justify-center items-center gap-2">
                                <FaSquareXTwitter /> Twitter
                            </Link>
                            <Link href="https://github.com/learnwithsatyam" target="_blank" className="text-sm text-muted-foreground hover:text-orange-500 flex justify-center items-center gap-2">
                                <FaGithub /> GitHub
                            </Link>
                            <Link href="https://linkedin.com/in/satyam-shivhare" target="_blank" className="text-sm text-muted-foreground hover:text-orange-500 flex justify-center items-center gap-2">
                                <FaLinkedin /> LinkedIn
                            </Link>
                            <div className="flex items-center justify-center">
                            </div>
                        </div>

                    </DrawerContent>
                </Drawer>
            </div>
        </nav>
    );
};
