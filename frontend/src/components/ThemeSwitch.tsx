import { Component, createSignal, onMount } from "solid-js";

const ThemeSwitch: Component = () => {
  const [isDark, setIsDark] = createSignal(false);

  onMount(() => {
    // Theme Logic
    const theme = localStorage.getItem("theme");
    if (theme === "dark") {
      setIsDark(true);
      document.documentElement.setAttribute("data-theme", "dark");
    } else if (theme === "light") {
      setIsDark(false);
      document.documentElement.setAttribute("data-theme", "light");
    }
  });

  const toggleTheme = () => {
    const nextIsDark = !isDark();
    setIsDark(nextIsDark);
    const theme = nextIsDark ? "dark" : "light";
    document.documentElement.setAttribute("data-theme", theme);
    localStorage.setItem("theme", theme);
  };

  return (
    <label class="swap swap-rotate btn btn-circle">
      {/* this hidden checkbox controls the state */}
      <input
        type="checkbox"
        class="theme-controller"
        value="dark"
        checked={isDark()}
        onChange={toggleTheme}
      />

      {/* sun icon */}
      <svg
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        stroke-width="1.5"
        stroke="currentColor"
        class="swap-on size-6"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M12 3v2.25m6.364.386-1.591 1.591M21 12h-2.25m-.386 6.364-1.591-1.591M12 18.75V21m-4.773-4.227-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z"
        />
      </svg>

      {/* moon icon */}
      <svg
        class="swap-off fill-current size-6"
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
      >
        <path
          d="M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z"
        />
      </svg>
    </label>
  );
};

export default ThemeSwitch;
