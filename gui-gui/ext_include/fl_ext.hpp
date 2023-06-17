#ifndef FL_EXT_HPP
#define FL_EXT_HPP

#include <FL/Fl.H>
#include <FL/fl_draw.H>
#include <FL/Fl_Widget.H>
#include <FL/Fl_Button.H>

#include <string>
#include <functional>

inline float fl_ext_dpi_scale = 1.5f;
#define sdpi(pixel) static_cast<int>(fl_ext_dpi_scale * pixel)
using Flcb = std::function<void(Fl_Widget* widget)>;

inline const int box_count = 6;
enum Ext_Box
{
    BTN_UP_BOX = 0,
    BTN_DOWN_BOX,
    BTN_HOVER_BOX,
    BTN_FRAME_BOX,
    INPUT_IDLE_BOX,
    INPUT_ACTIVE_BOX
};
class Fl_Ext_Color
{
  private:
    unsigned char r_;
    unsigned char g_;
    unsigned char b_;

  public:
    inline Fl_Ext_Color();
    inline Fl_Ext_Color(unsigned char r, unsigned char g, unsigned char b);
    inline Fl_Ext_Color(unsigned int hex, bool index = false);

    inline operator Fl_Color();
    inline int r() const { return r_; }
    inline int g() const { return g_; }
    inline int b() const { return b_; }
    inline void r(unsigned char r) { r_ = r; }
    inline void g(unsigned char g) { g_ = g; }
    inline void b(unsigned char b) { b_ = b; }

    inline void set(unsigned char r, unsigned char g, unsigned char b);
    inline void set(unsigned int hex);
    inline void index(unsigned int index);
};

inline void flcb_bridge(Fl_Widget* widget, void* func_ptr)
{
    (*static_cast<Flcb*>(func_ptr))(widget);
}
inline Fl_Boxtype fl_ext_box(int index)
{
    return static_cast<Fl_Boxtype>(FL_FREE_BOXTYPE + index);
}

namespace
{
    inline void ext_btn_up_box(int x, int y, int w, int h, Fl_Color c)
    {
        fl_color(Fl_Ext_Color(0xE1E1E1));
        fl_rectf(x, y, w, h);
        fl_color(Fl_Ext_Color(0xADADAD));
        fl_rect(x, y, w, h);
    }

    inline void ext_btn_down_box(int x, int y, int w, int h, Fl_Color c)
    {
        fl_color(Fl_Ext_Color(0xCCE4F7));
        fl_rectf(x, y, w, h);
        fl_color(Fl_Ext_Color(0x005499));
        fl_rect(x, y, w, h);
    }

    inline void ext_btn_hover_box(int x, int y, int w, int h, Fl_Color c)
    {
        fl_color(Fl_Ext_Color(0xE5F1FB));
        fl_rectf(x, y, w, h);
        fl_color(Fl_Ext_Color(0x0078D7));
        fl_rect(x, y, w, h);
    }

    inline void ext_frame_box(int x, int y, int w, int h, Fl_Color c)
    {
        fl_color(Fl_Ext_Color(0x7A7A7A));
        fl_rect(x, y, w, h);
    }

    inline void ext_input_idle_box(int x, int y, int w, int h, Fl_Color c)
    {
        fl_color(Fl_Ext_Color(0xFFFFFF));
        fl_rectf(x, y, w, h);
        fl_color(Fl_Ext_Color(0x7A7A7A));
        fl_rect(x, y, w, h);
    }

    inline void ext_input_active_box(int x, int y, int w, int h, Fl_Color c)
    {
        fl_color(Fl_Ext_Color(0xFFFFFF));
        fl_rectf(x, y, w, h);
        fl_color(Fl_Ext_Color(0x0078D7));
        fl_rect(x, y, w, h);
    }

}; // namespace

class Style_Initializer
{
  public:
    inline Style_Initializer()
    {
        Fl::set_boxtype(fl_ext_box(BTN_UP_BOX), ::ext_btn_up_box, 0, 0, 0, 0);
        Fl::set_boxtype(fl_ext_box(BTN_DOWN_BOX), ::ext_btn_down_box, 0, 0, 0, 0);
        Fl::set_boxtype(fl_ext_box(BTN_HOVER_BOX), ::ext_btn_hover_box, 0, 0, 0, 0);
        Fl::set_boxtype(fl_ext_box(BTN_FRAME_BOX), ::ext_frame_box, 0, 0, 0, 0);
        Fl::set_boxtype(fl_ext_box(INPUT_IDLE_BOX), ::ext_input_idle_box, 0, 0, 0, 0);
        Fl::set_boxtype(fl_ext_box(INPUT_ACTIVE_BOX), ::ext_input_active_box, 0, 0, 0, 0);
    }
};

// widget extensions
template <typename Wd_B>
class Fl_Ext_Attrib
{
  private:
    Wd_B* widget_ = nullptr;

  public:
    inline Fl_Ext_Attrib(Wd_B* widget);
    inline Fl_Ext_Attrib(Wd_B& widget);

    // pointer to the widget itself
    inline Wd_B& widget() { return *widget_; }
    inline Wd_B* widget_ptr() { return widget_; }
    inline operator Wd_B&() { return *widget_; }
    inline operator Wd_B*() { return widget_; }

    inline bool operator==(Fl_Ext_Attrib<Wd_B>&& target);

    inline Fl_Boxtype normal_box() const;
    inline void normal_box(Fl_Boxtype new_box);

    inline std::string label();
    inline unsigned int label_shortcut();
    inline void label(std::string text);
    inline std::string tooltip();
    inline void tooltip(std::string text);

    inline Fl_Ext_Color color() const;
    inline void color(Fl_Ext_Color color);

    inline Fl_Ext_Color selection_color() const;
    inline void selection_color(Fl_Ext_Color color);

    inline Fl_Ext_Color labelcolor() const;
    inline void labelcolor(Fl_Ext_Color color);

    inline Flcb* callback() const;
    inline void callback(Flcb* cb);
    inline void callback(Flcb& cb);
};
template <typename Wd_B>
Fl_Ext_Attrib<Wd_B> make_attrib(Wd_B& widget)
{
    return Fl_Ext_Attrib<Wd_B>(widget);
}
template <typename Wd_B>
Fl_Ext_Attrib<Wd_B> make_attrib(Wd_B* widget)
{
    return Fl_Ext_Attrib<Wd_B>(widget);
}

template <typename Wd_T>
class Fl_Ext : public Wd_T
{
  public:
    Fl_Ext_Attrib<Wd_T> attrib;
    inline Fl_Ext(int X, int Y, int W, int H, const char* L = 0);

    void callback(Fl_Callback) = delete;
    void default_callback() = delete;

    void callback(Flcb* func);
    void callback(Flcb& func);
    Flcb* callback();
};

// implementations
inline Fl_Ext_Color::Fl_Ext_Color()
{
    r_ = 255;
    g_ = 255;
    b_ = 255;
}

inline Fl_Ext_Color::Fl_Ext_Color(unsigned char r, unsigned char g, unsigned char b)
{
    r_ = r;
    g_ = g;
    b_ = b;
}

inline Fl_Ext_Color::Fl_Ext_Color(unsigned int hex, bool index)
{
    if (index)
    {
        Fl::get_color(hex, r_, g_, b_);
    }
    else
    {
        r_ = hex / 0x10000;
        g_ = ((hex / 0x100) % 0x100);
        b_ = hex % 0x100;
    }
}

inline Fl_Ext_Color::operator Fl_Color()
{
    return fl_rgb_color(r_, g_, b_);
};

inline void Fl_Ext_Color::set(unsigned char r, unsigned char g, unsigned char b)
{
    r_ = r;
    g_ = g;
    b_ = b;
}

inline void Fl_Ext_Color::set(unsigned int hex)
{
    r_ = hex / 0x10000;
    g_ = ((hex / 0x100) % 0x100);
    b_ = hex % 0x100;
}

inline void Fl_Ext_Color::index(unsigned int index)
{
    Fl::get_color(index, r_, g_, b_);
}

//////////////////////////////Blank//////////////////////////////

template <typename Wd_B>
inline Fl_Ext_Attrib<Wd_B>::Fl_Ext_Attrib(Wd_B* widget) : widget_(widget)
{
}

template <typename Wd_B>
inline Fl_Ext_Attrib<Wd_B>::Fl_Ext_Attrib(Wd_B& widget) : widget_(&widget)
{
}

template <typename Wd_B>
inline bool Fl_Ext_Attrib<Wd_B>::operator==(Fl_Ext_Attrib<Wd_B>&& target)
{
    return (target.widget_ == widget_) ? true : false;
}

// box attrib extention
template <typename Wd_B>
inline Fl_Boxtype Fl_Ext_Attrib<Wd_B>::normal_box() const
{
    return widget_->box();
}

template <typename Wd_B>
inline void Fl_Ext_Attrib<Wd_B>::normal_box(Fl_Boxtype new_box)
{
    widget_->box(new_box);
}

// label and tooltip extensions
template <typename Wd_B>
inline std::string Fl_Ext_Attrib<Wd_B>::label()
{
    return widget_->label();
}

template <typename Wd_B>
inline unsigned int Fl_Ext_Attrib<Wd_B>::label_shortcut()
{
    return widget_->label_shortcut(widget_->label());
}

template <typename Wd_B>
inline void Fl_Ext_Attrib<Wd_B>::label(std::string text)
{
    widget_->copy_label(text.c_str());
}

template <typename Wd_B>
inline std::string Fl_Ext_Attrib<Wd_B>::tooltip()
{
    return widget_->tooltip();
}

template <typename Wd_B>
inline void Fl_Ext_Attrib<Wd_B>::tooltip(std::string text)
{
    widget_->copy_tooltip(text.c_str());
}

// color attrib extention
template <typename Wd_B>
inline Fl_Ext_Color Fl_Ext_Attrib<Wd_B>::color() const
{
    return {widget_->color(), true};
}

template <typename Wd_B>
inline void Fl_Ext_Attrib<Wd_B>::color(Fl_Ext_Color color)
{
    widget_->color(color);
}

template <typename Wd_B>
inline Fl_Ext_Color Fl_Ext_Attrib<Wd_B>::selection_color() const
{
    return {widget_->selection_color(), true};
}

template <typename Wd_B>
inline void Fl_Ext_Attrib<Wd_B>::selection_color(Fl_Ext_Color color)
{
    widget_->selection_color(color);
}

template <typename Wd_B>
inline Fl_Ext_Color Fl_Ext_Attrib<Wd_B>::labelcolor() const
{
    return {widget_->labelcolor(), true};
}

template <typename Wd_B>
inline void Fl_Ext_Attrib<Wd_B>::labelcolor(Fl_Ext_Color color)
{
    widget_->labelcolor(color);
}

// callback extension
template <typename Wd_B>
inline Flcb* Fl_Ext_Attrib<Wd_B>::callback() const
{
    return static_cast<Flcb*>(widget_->user_data());
}

template <typename Wd_B>
inline void Fl_Ext_Attrib<Wd_B>::callback(Flcb* cb)
{
    widget_->callback(flcb_bridge, cb);
}

template <typename Wd_B>
inline void Fl_Ext_Attrib<Wd_B>::callback(Flcb& cb)
{
    widget_->callback(flcb_bridge, &cb);
}

template <typename Wd_T>
inline Fl_Ext<Wd_T>::Fl_Ext(int X, int Y, int W, int H, const char* L) //
    : Wd_T(sdpi(X), sdpi(Y), sdpi(W), sdpi(H), L),                     //
      attrib(this)
{
}

template <typename Wd_T>
inline void Fl_Ext<Wd_T>::callback(Flcb* func)
{
    attrib.callback(func);
}

template <typename Wd_T>
inline void Fl_Ext<Wd_T>::callback(Flcb& func)
{
    attrib.callback(func);
}

template <typename Wd_T>
inline Flcb* Fl_Ext<Wd_T>::callback()
{
    return attrib.callback();
}

#endif // FL_EXT_HPP
