import { Component, createSignal, Show } from "solid-js";
import { isAuthModalOpen, setIsAuthModalOpen, setApiKey } from "../lib/store";

const AuthModal: Component = () => {
    const [inputKey, setInputKey] = createSignal("");

    const handleSave = () => {
        const key = inputKey().trim();
        if (key) {
            setApiKey(key);
            localStorage.setItem("api_key", key);
            setIsAuthModalOpen(false);
            // Reload to re-fetch tables with the new key
            window.location.reload();
        }
    };

    return (
        <Show when={isAuthModalOpen()}>
            <div class="modal modal-open bg-black/50">
                <div class="modal-box">
                    <h3 class="font-bold text-lg">Authentication Required</h3>
                    <p class="py-4">Please enter the API Key to access the database.</p>
                    <div class="form-control w-full">
                        <input
                            type="password"
                            placeholder="API Key"
                            class="input input-bordered w-full"
                            value={inputKey()}
                            onInput={(e) => setInputKey(e.currentTarget.value)}
                            onKeyDown={(e) => e.key === "Enter" && handleSave()}
                        />
                    </div>
                    <div class="modal-action">
                        <button class="btn btn-primary" onClick={handleSave}>Save & Reload</button>
                    </div>
                </div>
            </div>
        </Show>
    );
};

export default AuthModal;
