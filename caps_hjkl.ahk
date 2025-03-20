CapsLock::
{
    KeyWait("CapsLock", "T0.2")  ; 等待 200ms，判斷是否為組合鍵
    if (A_ThisHotkey = "CapsLock")  ; 如果沒有其他鍵一起按，則切換 CapsLock 狀態
        SetCapsLockState(!GetKeyState("CapsLock", "T"))  
}

; CapsLock + HJKL 方向鍵
CapsLock & h::Send "{Left}"
CapsLock & j::Send "{Down}"
CapsLock & k::Send "{Up}"
CapsLock & l::Send "{Right}"
