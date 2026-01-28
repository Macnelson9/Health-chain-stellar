import Image from "next/image";

export default function Footer() {
  return (
    <footer className="w-full bg-brand-footer text-white pt-20 pb-10 font-dmsans">
      <div className="max-w-[1440px] mx-auto px-6 xl:px-[188px]">
        
        {/* --- ROW 1 --- */}
        <div className="flex flex-col md:flex-row items-center justify-between border-b border-gray-700 pb-12 mb-12 gap-8 md:gap-0">
            <div className="w-[60px] h-[60px] bg-white rounded-full flex items-center justify-center">
                <Image src="/logo-drop.svg" alt="Logo" width={30} height={30} />
            </div>

            <div className="flex flex-col md:flex-row items-center gap-8 text-center md:text-left">
                <span className="text-[22px] font-normal">Ready to get started?</span>
                <button className="bg-brand-loginBtn text-[#fffbfb] w-[185px] h-[45px] rounded font-roboto font-bold text-[20px] shadow-lg hover:opacity-90">
                    Become A Donor
                </button>
            </div>
        </div>

        {/* --- ROW 2 --- */}
        <div className="flex flex-col xl:flex-row justify-between items-center xl:items-start gap-20 mb-20 text-center xl:text-left">
            
            {/* Newsletter */}
            <div className="flex flex-col items-center xl:items-start gap-6">
                <p className="w-full md:w-[165px] font-normal text-[22px] leading-[100%] tracking-[-0.3px]">
                    Subscribe to our newsletter
                </p>
                <div className="flex items-center bg-brand-footer border border-[#4b4949] rounded-lg overflow-hidden w-[289px] h-[51px]">
                    <input 
                        type="email" 
                        placeholder="Email address" 
                        className="bg-brand-footer text-white px-4 w-full h-full border-none focus:outline-none placeholder-gray-400 font-poppins text-[15px]" 
                    />
                    <button className="bg-brand-loginBtn w-[50px] h-[50px] flex items-center justify-center hover:opacity-90">
                         <span className="text-white text-xl font-bold">&gt;</span>
                    </button>
                </div>
            </div>

            {/* Links */}
            <div className="flex flex-col md:flex-row gap-10 md:gap-20">
                <div>
                    <h3 className="text-[#bfbfbf] text-[16px] mb-6">Services</h3>
                    <ul className="space-y-4 text-[15px] font-normal text-gray-300">
                        <li><a href="#" className="hover:text-brand-loginBtn">Email Marketing</a></li>
                        <li><a href="#" className="hover:text-brand-loginBtn">Campaigns</a></li>
                        <li><a href="#" className="hover:text-brand-loginBtn">Branding</a></li>
                        <li><a href="#" className="hover:text-brand-loginBtn">Offline</a></li>
                    </ul>
                </div>
                <div>
                    <h3 className="text-[#bfbfbf] text-[16px] mb-6">About</h3>
                    <ul className="space-y-4 text-[15px] font-normal text-gray-300">
                        <li><a href="#" className="hover:text-brand-loginBtn">Our Story</a></li>
                        <li><a href="#" className="hover:text-brand-loginBtn">Benefits</a></li>
                        <li><a href="#" className="hover:text-brand-loginBtn">Team</a></li>
                        <li><a href="#" className="hover:text-brand-loginBtn">Careers</a></li>
                    </ul>
                </div>
                <div>
                    <h3 className="text-[#bfbfbf] text-[16px] mb-6">Help</h3>
                    <ul className="space-y-4 text-[15px] font-normal text-gray-300">
                        <li><a href="#" className="hover:text-brand-loginBtn">FAQs</a></li>
                        <li><a href="#" className="hover:text-brand-loginBtn">Contact Us</a></li>
                    </ul>
                </div>
            </div>
        </div>

        {/* --- ROW 3 --- */}
        <div className="flex flex-col md:flex-row justify-between items-center pt-8 gap-6 md:gap-0">
            <div className="flex gap-8 text-[15px] text-gray-300">
                <a href="#" className="hover:text-white">Terms & Conditions</a>
                <a href="#" className="hover:text-white">Privacy Policy</a>
            </div>

            <div className="flex gap-6">
                <a href="#" className="w-[22px] h-[22px] text-white hover:text-brand-loginBtn"><svg fill="currentColor" viewBox="0 0 24 24"><path d="M18 2h-3a5 5 0 00-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 011-1h3z"></path></svg></a>
                <a href="#" className="w-[22px] h-[22px] text-white hover:text-brand-loginBtn"><svg fill="currentColor" viewBox="0 0 24 24"><path d="M23 3a10.9 10.9 0 01-3.14 1.53 4.48 4.48 0 00-7.86 3v1A10.66 10.66 0 013 4s-4 9 5 13a11.64 11.64 0 01-7 2c9 5 20 0 20-11.5a4.5 4.5 0 00-.08-.83A7.72 7.72 0 0023 3z"></path></svg></a>
                <a href="#" className="w-[22px] h-[22px] text-white hover:text-brand-loginBtn"><svg fill="none" stroke="currentColor" strokeWidth="2" viewBox="0 0 24 24"><rect x="2" y="2" width="20" height="20" rx="5" ry="5"></rect><path d="M16 11.37A4 4 0 1112.63 8 4 4 0 0116 11.37z"></path><line x1="17.5" y1="6.5" x2="17.51" y2="6.5"></line></svg></a>
            </div>
        </div>

      </div>
    </footer>
  );
}