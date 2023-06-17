#include "Fl_Ext.hpp"

#include <iostream>
// Hover extensions

template <typename Wd_T> //
class Fl_Hover_Ext : public Fl_Ext<Wd_T>
{
  private:
    Fl_Boxtype normal_box_;
    Fl_Boxtype hover_box_;

    Fl_Ext_Color hover_color_;
    Fl_Ext_Color normal_color_;

  public:
    inline Fl_Hover_Ext(int X, int Y, int W, int H, const char* L = 0);

    inline Fl_Boxtype hover_box() const;
    inline void hover_box(Fl_Boxtype new_box);

    void box() = delete;
    inline Fl_Boxtype normal_box() const;
    inline void normal_box(Fl_Boxtype new_box);

    inline Fl_Ext_Color hover_color() const;
    inline void hover_color(Fl_Ext_Color new_color);

    inline Fl_Ext_Color normal_color() const;
    inline void normal_color(Fl_Ext_Color new_color);

    inline int handle(int event);
};

//////////////////////////////Blank//////////////////////////////

template <typename Wd_T>
inline Fl_Hover_Ext<Wd_T>::Fl_Hover_Ext(int X, int Y, int W, int H, const char* L) //
    : Fl_Ext<Wd_T>(X, Y, W, H, L)                                                  //
{
    hover_box(Fl_Ext<Wd_T>::box());
    normal_box(Fl_Ext<Wd_T>::box());

    hover_color({Fl_Ext<Wd_T>::color(), true});
    normal_color({Fl_Ext<Wd_T>::color(), true});
}
template <typename Wd_T>
inline Fl_Boxtype Fl_Hover_Ext<Wd_T>::hover_box() const
{
    return hover_box_;
}

template <typename Wd_T>
inline void Fl_Hover_Ext<Wd_T>::hover_box(Fl_Boxtype new_box)
{
    hover_box_ = new_box;
}

template <typename Wd_T>
inline Fl_Boxtype Fl_Hover_Ext<Wd_T>::normal_box() const
{
    return normal_box_;
}

template <typename Wd_T>
inline void Fl_Hover_Ext<Wd_T>::normal_box(Fl_Boxtype new_box)
{
    normal_box_ = new_box;
    Fl_Ext<Wd_T>::box(normal_box_);
}

template <typename Wd_T>
inline Fl_Ext_Color Fl_Hover_Ext<Wd_T>::hover_color() const
{
    return hover_color_;
}

template <typename Wd_T>
inline void Fl_Hover_Ext<Wd_T>::hover_color(Fl_Ext_Color new_color)
{
    hover_color_ = new_color;
};

template <typename Wd_T>
inline Fl_Ext_Color Fl_Hover_Ext<Wd_T>::normal_color() const
{
    return normal_color_;
}

template <typename Wd_T>
inline void Fl_Hover_Ext<Wd_T>::normal_color(Fl_Ext_Color new_color)
{
    normal_color_ = new_color;
    Fl_Ext<Wd_T>::color(normal_color_);
}

template <typename Wd_T>
inline int Fl_Hover_Ext<Wd_T>::handle(int event)
{
    int result = Fl_Ext<Wd_T>::handle(event);

    switch (event)
    {
        case FL_ENTER:
        {
            Fl_Ext<Wd_T>::box(hover_box_);
            Fl_Ext<Wd_T>::color(hover_color_);
            Fl_Ext<Wd_T>::redraw();
            break;
        }
        case FL_LEAVE:
        {
            Fl_Ext<Wd_T>::box(normal_box_);
            Fl_Ext<Wd_T>::color(normal_color_);
            Fl_Ext<Wd_T>::redraw();
            break;
        }
    }

    return result;
}
